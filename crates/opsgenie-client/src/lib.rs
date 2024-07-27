use crate::api::response::ApiResponse;
use api::response::ApiError;
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

pub mod api;
pub mod limits;
pub mod pagination;
pub mod query_builder;

/// The Opsgenie API version to use.
const API_VERSION: &str = "v2/";

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Client error occurred: {0}")]
    Client(#[from] reqwest::Error),
    #[error("Request failed: {0}")]
    Request(#[from] crate::api::response::ApiError),
}

pub type Result<T> = ::core::result::Result<T, ClientError>;

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

    pub(crate) async fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<ApiResponse<R>> {
        let url = self.url(path);
        let request = self.client.post(url).json(body);
        self.perform_request(request).await
    }

    pub(crate) async fn get<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        query: &T,
    ) -> Result<ApiResponse<R>> {
        let url = self.url(path);
        let request = self.client.get(url).query(query);
        self.perform_request(request).await
    }

    fn url(&self, path: &str) -> url::Url {
        self.base_url
            .join(path)
            .unwrap_or_else(|err| panic!("Invalid path provided: {path}: {err}"))
    }

    async fn perform_request<R: DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<ApiResponse<R>> {
        let request = request.header("Authorization", format!("GenieKey {}", self.api_key));
        let response = request.send().await?;
        // TODO: If you get 503, you should retry the request, but if 429 you should wait a bit then retry the request *
        if response.status().is_success() {
            let response: ApiResponse<R> = response.json().await?;
            Ok(response)
        } else {
            // TODO: Handle rate limiting.
            // 2024-07-19T17:23:00.029276Z ERROR opsgenie_client
            // Failed to deserialize response Object {"message": String("You are making too many requests! To avoid errors, we recommend you limit requests."),
            // "requestId": String("831e6aca-2dd3-475b-9acc-bd385b2f5e7a"), "took": Number(0.002)}: Error("missing field `data`", line: 0, column: 0)

            let error: ApiError = response.json().await?;
            Err(ClientError::Request(error))
        }
    }
}
