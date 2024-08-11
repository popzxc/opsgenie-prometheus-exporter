use std::{io::BufRead as _, time::Duration};

use anyhow::Context as _;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub opsgenie_base_url: url::Url,
    pub opsgenie_api_key: String,
    #[serde(default = "Config::default_prometheus_port")]
    pub prometheus_port: u16,
    #[serde(default = "Config::default_log_format")]
    pub log_format: String,
    #[serde(default = "Config::default_polling_interval_secs")]
    pub polling_interval_secs: u64,
}

impl Config {
    pub fn load(dotenv_path: &str) -> anyhow::Result<Self> {
        // Check if file exists. If not, load values from env.
        let config = if std::path::Path::new(dotenv_path).exists() {
            let file = std::fs::File::open(dotenv_path).context("Can't open config file")?;
            // Create iterator over `KEY=VAL` pairs.
            // Somewhat ugly but whatever.
            let kv: Vec<_> = std::io::BufReader::new(file)
                .lines()
                .map(|line| {
                    line.context("Can't read line").and_then(|line| {
                        let mut parts = line.splitn(2, '=');
                        let key = parts.next().context("No key")?.to_string();
                        let value = parts.next().context("No value")?.to_string();
                        Ok((key, value))
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()?;

            envy::from_iter(kv).context("Malformed .env file")?
        } else {
            envy::from_env().context("Failed to load values from env")?
        };
        Ok(config)
    }

    fn default_log_format() -> String {
        "plain".into()
    }

    fn default_prometheus_port() -> u16 {
        3652
    }

    fn default_polling_interval_secs() -> u64 {
        60 * 5
    }

    pub fn polling_interval(&self) -> Duration {
        Duration::from_secs(self.polling_interval_secs)
    }
}
