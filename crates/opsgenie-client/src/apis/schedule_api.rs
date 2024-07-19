use crate::response::{schedule::Schedule, ApiResponse};

#[derive(Debug)]
pub struct ScheduleApi<'a>(pub(crate) &'a crate::OpsgenieClient);

impl<'a> ScheduleApi<'a> {
    pub async fn list_schedules(&self) -> anyhow::Result<ApiResponse<Vec<Schedule>>> {
        self.0.get_json("schedules", &()).await
    }
}
