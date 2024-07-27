use crate::api::response::ApiResponse;

pub mod response;

#[derive(Debug)]
pub struct ScheduleApi<'a>(pub(crate) &'a crate::OpsgenieClient);

impl<'a> ScheduleApi<'a> {
    pub async fn list_schedules(
        &self,
    ) -> crate::Result<ApiResponse<Vec<self::response::Schedule>>> {
        self.0.get("schedules", &()).await
    }
}
