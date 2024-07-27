use crate::{api::response::ApiResponse, query_builder::ToFilter};

pub mod response;

#[derive(Debug)]
pub struct AlertApi<'a>(pub(crate) &'a crate::OpsgenieClient);

impl<'a> AlertApi<'a> {
    pub async fn count(
        &self,
        query: impl ToFilter,
    ) -> anyhow::Result<ApiResponse<self::response::Count>> {
        let query = query.to_filter();
        tracing::debug!(query=%query, "Sending query");
        self.0.get_json("alerts/count", &[("query", query)]).await
    }

    pub async fn list(
        &self,
        query: impl ToFilter,
        limit: Option<u32>,
    ) -> anyhow::Result<ApiResponse<Vec<self::response::Alert>>> {
        let limit = limit.unwrap_or(100);
        let query = query.to_filter();
        tracing::debug!(query=%query, "Sending query");
        self.0
            .get_json("alerts", &[("query", query), ("limit", limit.to_string())])
            .await
    }
}
