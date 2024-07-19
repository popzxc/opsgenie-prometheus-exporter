use response::{
    schedule::{OnCallRecipients, Schedule},
    ApiResponse,
};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

pub mod apis;
pub mod query_builder;
pub mod response;

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

    pub fn on_call(&self) -> apis::OnCallApi<'_> {
        apis::OnCallApi(self)
    }

    pub fn schedule(&self) -> apis::ScheduleApi<'_> {
        apis::ScheduleApi(self)
    }

    pub fn alert(&self) -> apis::AlertApi<'_> {
        apis::AlertApi(self)
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
        Ok(response)
    }
}
