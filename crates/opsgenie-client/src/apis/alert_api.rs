use crate::{query_builder::ToFilter, response::ApiResponse};

#[derive(Debug)]
pub struct AlertApi<'a>(pub(crate) &'a crate::OpsgenieClient);

impl<'a> AlertApi<'a> {
    pub async fn count(
        &self,
        query: impl ToFilter,
    ) -> anyhow::Result<ApiResponse<crate::response::alert::Count>> {
        let query = query.to_filter();
        tracing::debug!(query=%query, "Sending query");
        self.0.get_json("alerts/count", &[("query", query)]).await
    }
}
