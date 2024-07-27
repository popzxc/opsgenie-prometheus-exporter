use crate::api::response::ApiResponse;

pub mod response;

#[derive(Debug)]
pub struct OnCallApi<'a>(pub(crate) &'a crate::OpsgenieClient);

impl<'a> OnCallApi<'a> {
    pub async fn whoisoncall(
        &self,
        schedule_id: &str,
    ) -> crate::Result<ApiResponse<self::response::OnCallRecipients>> {
        self.0
            .get(
                &format!("schedules/{}/on-calls", schedule_id),
                &[("flat", true)],
            )
            .await
    }
}
