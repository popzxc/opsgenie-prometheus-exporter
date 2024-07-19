use opsgenie_client::{
    query_builder::{Query, ToFilter as _},
    OpsgenieClient,
};
use serde::Deserialize;
use std::{collections::HashMap, time::Duration};
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
        60
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
        // Get all schedules
        let schedules = self.client.schedule().schedules().await?;

        // Sort them by team.
        let mut team_schedules = HashMap::new();
        for schedule in schedules.data {
            let team = schedule.owner_team.name.clone();
            team_schedules
                .entry(team)
                .or_insert_with(Vec::new)
                .push(schedule);
        }

        for (team, schedules) in team_schedules {
            tracing::info!("Team: {}", team.clone());
            for schedule in schedules {
                let on_call = self.client.on_call().whoisoncall(&schedule.id).await?;

                tracing::info!("  - Schedule {}:", schedule.name);
                for recipient in on_call.data.on_call_recipients {
                    tracing::info!("    - {}", recipient);
                }
                // let mut labels = HashMap::new();
                // labels.insert("schedule_id".to_string(), schedule.id);
                // labels.insert("schedule_name".to_string(), schedule.name);
                // metrics::ON_CALL.with_label_values(labels).set(on_call.on_call_recipients.len() as f64);
            }
            let open_alerts = self
                .client
                .alert()
                .count(Query::new("team", team).and(Query::new("status", "open")))
                .await?;
            tracing::info!("  - Open alerts: {}", open_alerts.data.count);
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
