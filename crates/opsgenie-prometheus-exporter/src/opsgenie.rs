use url::Url;

/// The Opsgenie API version to use.
const API_VERSION: &str = "v2";

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

    async fn perform_request(
        &self,
        request: reqwest::RequestBuilder,
    ) -> anyhow::Result<reqwest::Response> {
        let request = request.header("Authorization", format!("GenieKey {}", self.api_key));
        let response = request.send().await?;
        Ok(response)
    }
}
