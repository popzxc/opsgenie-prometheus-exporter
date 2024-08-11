use std::time::Duration;

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
