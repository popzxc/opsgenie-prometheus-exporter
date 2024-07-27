use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub result: Option<String>,
    pub data: T,
    pub took: Option<f64>,
    pub expandable: Option<serde_json::Value>,
    pub request_id: String,
}
