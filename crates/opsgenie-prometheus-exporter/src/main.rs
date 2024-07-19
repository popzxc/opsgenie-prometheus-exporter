use metrics::{OnCallStatus, METRICS};
use opsgenie_client::{
    query_builder::{Query, ToFilter as _},
    OpsgenieClient,
};
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    time::{Duration, SystemTime},
};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};
use url::Url;
use vise_exporter::MetricsExporter;

mod metrics;

#[derive(Debug, Deserialize)]
struct Config {
    pub opsgenie_base_url: url::Url,
    pub opsgenie_api_key: String,
    pub prometheus_port: u16,
    pub log_format: String,
    #[serde(default = "Config::default_polling_interval_secs")]
    pub polling_interval_secs: u64,
}

impl Config {
    fn default_polling_interval_secs() -> u64 {
        60 * 5
    }

    fn polling_interval(&self) -> Duration {
        Duration::from_secs(self.polling_interval_secs)
    }
}

fn init_tracing(json: bool) {
    if json {
        let timer = tracing_subscriber::fmt::time::UtcTime::rfc_3339();
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_file(true)
                    .with_line_number(true)
                    .with_timer(timer)
                    .json(),
            )
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    }
}

struct OpsgenieUpdater {
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
                METRICS.alerts[&(team.clone(), "total", priority.clone())].set(open.data.count);
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: Config = envy::from_env()?;
    init_tracing(config.log_format.to_ascii_lowercase() == "json");

    tracing::info!("Starting up");
    let polling_interval = config.polling_interval();
    let updater = OpsgenieUpdater::new(
        config.opsgenie_base_url,
        config.opsgenie_api_key,
        polling_interval,
    );
    let updater_task = tokio::spawn(updater.run());

    let exporter = MetricsExporter::default();
    let bind_address = format!("0.0.0.0:{}", config.prometheus_port)
        .parse()
        .unwrap();
    let prometheus_task = tokio::spawn(exporter.start(bind_address));

    tokio::select! {
        _ = updater_task => {
            tracing::error!("Updater task failed");
        }
        _ = prometheus_task => {
            tracing::error!("Prometheus task failed");
        }
    };

    Ok(())
}
