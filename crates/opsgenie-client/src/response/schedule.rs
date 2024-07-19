use serde::Deserialize;

use super::team::Team;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub id: String,
    pub name: String,
    pub owner_team: Team,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnCallRecipients {
    pub on_call_recipients: Vec<String>,
}
