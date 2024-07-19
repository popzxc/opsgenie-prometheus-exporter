use serde::{de::DeserializeOwned, Deserialize, Serialize};
use types::ApiResponse;
use url::Url;

pub mod query_builder;
pub mod types;

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

    pub async fn schedules(&self) -> anyhow::Result<ApiResponse<Vec<types::Schedule>>> {
        self.get_json("schedules").await
    }

    // pub async fn schedules(&self) -> anyhow::Result<ApiResponse<Vec<serde_json::Value>>> {
    //     self.get_json("schedules").await
    // }

    pub async fn whoisoncall(
        &self,
        schedule_id: &str,
    ) -> anyhow::Result<ApiResponse<types::OnCallRecipients>> {
        self.get_json(&format!("schedules/{}/on-calls?flat=true", schedule_id))
            .await
    }

    async fn post_json<T: Serialize, R: DeserializeOwned>(
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

    async fn get_json<R: DeserializeOwned>(&self, path: &str) -> anyhow::Result<ApiResponse<R>> {
        let url = self.base_url.join(path)?;
        let request = self.client.get(url);
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
        Ok(response)
    }
}
