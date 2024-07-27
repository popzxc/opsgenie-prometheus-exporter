use std::{collections::HashMap, fmt};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct RequestId(pub String);

// TODO: metadata from headers (e.g. rate limiting)
// https://docs.opsgenie.com/docs/api-rate-limiting

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub result: Option<String>,
    pub data: T,
    pub took: f64,
    pub expandable: Option<serde_json::Value>,
    pub message: Option<String>,
    pub request_id: RequestId,
}

#[derive(Debug, Clone, Deserialize, thiserror::Error)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub request_id: RequestId,
    pub message: String,
    pub took: f64,
    pub errors: HashMap<String, String>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}
