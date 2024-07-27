use crate::api::response::ApiResponse;

pub mod response;

#[derive(Debug)]
pub struct TeamApi<'a>(pub(crate) &'a crate::OpsgenieClient);

impl<'a> TeamApi<'a> {
    pub async fn list_teams(
        &self,
    ) -> crate::Result<ApiResponse<Vec<self::response::TeamDescriptor>>> {
        self.0.get("teams", &()).await
    }

    pub async fn get(&self, team_id: String) -> crate::Result<ApiResponse<self::response::Team>> {
        self.0.get(&format!("teams/{}", team_id), &()).await
    }
}
