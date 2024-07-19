use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub took: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expandable: Option<serde_json::Value>,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamMember {
    pub user: User,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: Option<String>,
    pub description: Option<String>,
    pub name: String,
    pub members: Option<Vec<TeamMember>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub id: String,
    pub name: String,
    pub owner_team: Team,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnCallRecipients {
    pub on_call_recipients: Vec<String>,
}
