use crate::response::{
    team::{Team, TeamDescriptor},
    ApiResponse,
};

#[derive(Debug)]
pub struct TeamApi<'a>(pub(crate) &'a crate::OpsgenieClient);

impl<'a> TeamApi<'a> {
    pub async fn list_teams(&self) -> anyhow::Result<ApiResponse<Vec<TeamDescriptor>>> {
        self.0.get_json("teams", &()).await
    }

    pub async fn get(&self, team_id: String) -> anyhow::Result<ApiResponse<Team>> {
        self.0.get_json(&format!("teams/{}", team_id), &()).await
    }
}
