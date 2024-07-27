use crate::api::response::ApiResponse;

pub mod response;

#[derive(Debug)]
pub struct TeamApi<'a>(pub(crate) &'a crate::OpsgenieClient);

impl<'a> TeamApi<'a> {
    pub async fn list_teams(
        &self,
    ) -> anyhow::Result<ApiResponse<Vec<self::response::TeamDescriptor>>> {
        self.0.get_json("teams", &()).await
    }

    pub async fn get(&self, team_id: String) -> anyhow::Result<ApiResponse<self::response::Team>> {
        self.0.get_json(&format!("teams/{}", team_id), &()).await
    }
}
