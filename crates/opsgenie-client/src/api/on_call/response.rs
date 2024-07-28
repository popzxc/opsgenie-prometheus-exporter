use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnCallRecipients {
    pub on_call_recipients: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::response_test;

    #[test]
    fn whoisoncall_flat_response() {
        let fixture = r#"
        {
    "data": {
        "_parent": {
            "id": "d875alp4-9b4e-4219-a803-0c26936d18de",
            "name": "ScheduleName",
            "enabled": true
        },
        "onCallRecipients": [
            "user4@opsgenie.com"
        ]
    },
    "took": 0.101,
    "requestId": "7f0alpde-3c67-455f-97ec-24754432d413"
}
        "#;
        response_test::<OnCallRecipients>(fixture);
    }
}
