use clap::Parser;
use config::Config;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};
use updater::OpsgenieUpdater;
use vise_exporter::MetricsExporter;

mod config;
mod metrics;
mod updater;

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

/// Exporter for Opsgenie data to Prometheus
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// Path to the `.env` file.
    /// If not provided, `.env` file will be attempted, and if it doesn't exist,
    /// environment variables will be used.
    #[arg(short, long)]
    env_file: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config = Config::load(cli.env_file.as_deref().unwrap_or(".env"))?;
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
