use crate::api::response::ApiResponse;
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

pub mod api;
pub mod query_builder;

/// The Opsgenie API version to use.
const API_VERSION: &str = "v2/";

#[derive(Debug)]
pub struct OpsgenieClient {
    base_url: Url,
    api_key: String,
    client: reqwest::Client,
}

impl OpsgenieClient {
    pub fn new(base_url: Url, api_key: String) -> Self {
        let client = reqwest::Client::new();
        let base_url = base_url.join(API_VERSION).unwrap();
        Self {
            base_url,
            api_key,
            client,
        }
    }

    pub fn on_call(&self) -> api::OnCallApi<'_> {
        api::OnCallApi(self)
    }

    pub fn schedule(&self) -> api::ScheduleApi<'_> {
        api::ScheduleApi(self)
    }

    pub fn alert(&self) -> api::AlertApi<'_> {
        api::AlertApi(self)
    }

    pub fn team(&self) -> api::TeamApi<'_> {
        api::TeamApi(self)
    }

    pub(crate) async fn post_json<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> anyhow::Result<ApiResponse<R>> {
        let url = self.base_url.join(path)?;
        let request = self.client.post(url).json(body);
        let response = self.perform_request(request).await?;
        let response = response.json::<serde_json::Value>().await?;

        match serde_json::from_value(response.clone()) {
            Ok(response) => Ok(response),
            Err(e) => {
                tracing::error!("Failed to deserialize response {:?}: {:?}", response, e);
                Err(anyhow::anyhow!(e))
            }
        }
    }

    pub(crate) async fn get_json<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        query: &T,
    ) -> anyhow::Result<ApiResponse<R>> {
        let url = self.base_url.join(path)?;
        let request = self.client.get(url).query(query);
        let response = self.perform_request(request).await?;
        let response = response.json::<serde_json::Value>().await?;

        match serde_json::from_value(response.clone()) {
            Ok(response) => Ok(response),
            Err(e) => {
                tracing::error!("Failed to deserialize response {:?}: {:?}", response, e);
                Err(anyhow::anyhow!(e))
            }
        }
    }

    async fn perform_request(
        &self,
        request: reqwest::RequestBuilder,
    ) -> anyhow::Result<reqwest::Response> {
        let request = request.header("Authorization", format!("GenieKey {}", self.api_key));
        let response = request.send().await?;
        // TODO: Handle rate limiting.
        // 2024-07-19T17:23:00.029276Z ERROR opsgenie_client
        // Failed to deserialize response Object {"message": String("You are making too many requests! To avoid errors, we recommend you limit requests."),
        // "requestId": String("831e6aca-2dd3-475b-9acc-bd385b2f5e7a"), "took": Number(0.002)}: Error("missing field `data`", line: 0, column: 0)
        Ok(response)
    }
}
