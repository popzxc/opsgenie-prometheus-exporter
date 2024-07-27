use serde::Deserialize;

use crate::api::team::response as team_response;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub id: String,
    pub name: String,
    pub owner_team: team_response::Team,
}
