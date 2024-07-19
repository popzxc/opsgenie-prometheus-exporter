use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Count {
    pub count: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Responder {
    pub id: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub ack_time: Option<u64>,
    pub close_time: Option<u64>,
    pub acknowledged_by: Option<String>,
    pub closed_by: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub id: String,
    pub tiny_id: String,
    pub message: String,
    pub status: String,
    pub acknowledged: bool,
    pub is_seen: bool,
    pub tags: Option<Vec<String>>,
    pub snoozed: bool,
    pub snoozed_until: Option<DateTime<FixedOffset>>,
    pub count: u64,
    pub last_occurred_at: DateTime<FixedOffset>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub source: String,
    pub owner: Option<String>,
    pub priority: String,
    pub responders: Vec<Responder>,
    pub report: Option<Report>,
}
