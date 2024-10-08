use crate::metrics::{OnCallStatus, METRICS};
use opsgenie_client::{
    query_builder::{Query, ToFilter as _},
    OpsgenieClient,
};
use std::{
    collections::{HashMap, HashSet},
    time::{Duration, SystemTime},
};
use url::Url;

#[derive(Debug)]
pub(crate) struct OpsgenieUpdater {
    client: OpsgenieClient,
    polling_interval: Duration,
}

impl OpsgenieUpdater {
    pub fn new(url: Url, api_key: String, polling_interval: Duration) -> Self {
        let client = OpsgenieClient::new(url, api_key);
        Self {
            client,
            polling_interval,
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        loop {
            self.step().await?;
            tokio::time::sleep(self.polling_interval).await;
        }
    }

    async fn step(&self) -> anyhow::Result<()> {
        // Get all teams
        let team_descriptors = self.client.team().list_teams().await?;
        let mut team_members = HashMap::new();
        for team_desc in team_descriptors.data {
            let team = self.client.team().get(team_desc.id).await?;
            team_members.insert(team_desc.name.clone(), HashSet::new());

            let Some(members) = team.data.members else {
                tracing::warn!("Team {} has no members", team_desc.name);
                continue;
            };
            for member in members {
                let Some(username) = member.user.username else {
                    tracing::warn!("Member has no username: {:?}", member);
                    continue;
                };

                team_members
                    .get_mut(&team_desc.name)
                    .unwrap()
                    .insert(username);
            }
        }

        for (team, members) in team_members.clone() {
            tracing::info!("Team: {}", team);
            for member in members {
                tracing::info!("  - {}", member);
            }
        }

        // Get all schedules
        let schedules = self.client.schedule().list_schedules().await?;

        // Sort them by team.
        let mut team_schedules = HashMap::new();
        // Also store all team members for each schedule to report who is on call.
        let mut not_on_call = HashMap::new();
        for schedule in schedules.data {
            let team = schedule.owner_team.name.clone();
            team_schedules
                .entry(team.clone())
                .or_insert_with(Vec::new)
                .push(schedule.clone());
            tracing::info!(
                "Adding members for schedule {}; team {}",
                schedule.name,
                team
            );
            not_on_call.insert(schedule.name, team_members[&team].clone());
        }

        for (team, schedules) in team_schedules.clone() {
            tracing::info!("Team: {}", team.clone());
            for schedule in schedules {
                let on_call = self.client.on_call().whoisoncall(&schedule.id).await?;

                tracing::info!("  - Schedule {}:", &schedule.name);
                for recipient in on_call.data.on_call_recipients {
                    tracing::info!("    - {}", recipient);
                    METRICS.on_call[&(team.clone(), schedule.name.clone(), recipient.clone())]
                        .set(OnCallStatus::OnCall as u64);
                    not_on_call
                        .entry(schedule.name.clone())
                        .and_modify(|members| {
                            members.remove(&recipient);
                        });
                }
                for team_member in &not_on_call[&schedule.name] {
                    METRICS.on_call[&(team.clone(), schedule.name.clone(), team_member.clone())]
                        .set(OnCallStatus::NotOnCall as u64);
                }
            }

            // TODO: Hack to avoid rate limiting.
            tokio::time::sleep(Duration::from_secs(5)).await;

            // TODO: filter out teams that had an alert in the last week.

            for priority in (1..=5).map(|p| format!("P{p}")) {
                let total = self
                    .client
                    .alert()
                    .count(
                        Query::new("team", team.clone())
                            .and(Query::new("priority", priority.clone())),
                    )
                    .await?;
                METRICS.alerts[&(team.clone(), "total", priority.clone())].set(total.data.count);
                tracing::info!(
                    "Team {} has {} alerts with priority {}",
                    team,
                    total.data.count,
                    priority
                );

                let open = self
                    .client
                    .alert()
                    .count(Query::new("team", team.clone()).and(
                        Query::new("priority", priority.clone()).and(Query::new("status", "open")),
                    ))
                    .await?;
                METRICS.alerts[&(team.clone(), "open", priority.clone())].set(open.data.count);
                tracing::info!(
                    "Team {} has {} open alerts with priority {}",
                    team,
                    open.data.count,
                    priority
                );
                if open.data.count > 0 {
                    // Get at most 100 alerts; should be representative enough.
                    const MAX_ALERTS_TO_FETCH: u32 = 100;
                    let alerts = self
                        .client
                        .alert()
                        .list(
                            Query::new("team", team.clone())
                                .and(Query::new("priority", priority.clone()))
                                .and(Query::new("status", "open")),
                            Some(MAX_ALERTS_TO_FETCH),
                        )
                        .await?;
                    for alert in alerts.data {
                        let unix_timestamp = alert.created_at.timestamp();
                        let alert_system_time =
                            SystemTime::UNIX_EPOCH + Duration::from_secs(unix_timestamp as u64);
                        let now = SystemTime::now();

                        if let Ok(duration) = now.duration_since(alert_system_time) {
                            METRICS.alert_duration[&(team.clone(), priority.clone())]
                                .observe(duration);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
