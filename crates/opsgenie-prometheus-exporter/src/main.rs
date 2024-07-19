use opsgenie_client::OpsgenieClient;
use serde::Deserialize;
use std::{collections::HashMap, env};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

mod metrics;

#[derive(Debug, Deserialize)]
struct Config {
    pub opsgenie_base_url: url::Url,
    pub opsgenie_api_key: String,
    pub prometheus_port: u16,
    pub log_format: String,
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: Config = envy::from_env()?;
    init_tracing(config.log_format.to_ascii_lowercase() == "json");

    tracing::info!("Starting up");
    let client = OpsgenieClient::new(config.opsgenie_base_url, config.opsgenie_api_key);

    // Get all schedules
    let schedules = client.schedules().await?;

    for schedule in schedules.data {
        let on_call = client.whoisoncall(&schedule.id).await?;

        println!("{}:", schedule.name);
        for recipient in on_call.data.on_call_recipients {
            println!("  - {}", recipient);
        }
        // let mut labels = HashMap::new();
        // labels.insert("schedule_id".to_string(), schedule.id);
        // labels.insert("schedule_name".to_string(), schedule.name);
        // metrics::ON_CALL.with_label_values(labels).set(on_call.on_call_recipients.len() as f64);
    }

    // metrics::OPSGENIE_REQUESTS.inc();

    Ok(())
}
