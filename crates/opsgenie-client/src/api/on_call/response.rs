use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnCallRecipients {
    pub on_call_recipients: Vec<String>,
}
