use crate::response::{schedule::OnCallRecipients, ApiResponse};

#[derive(Debug)]
pub struct OnCallApi<'a>(pub(crate) &'a crate::OpsgenieClient);

impl<'a> OnCallApi<'a> {
    pub async fn whoisoncall(
        &self,
        schedule_id: &str,
    ) -> anyhow::Result<ApiResponse<OnCallRecipients>> {
        self.0
            .get_json(
                &format!("schedules/{}/on-calls", schedule_id),
                &[("flat", true)],
            )
            .await
    }
}
