use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Count {
    pub count: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Responder {
    pub id: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub ack_time: Option<u64>,
    pub close_time: Option<u64>,
    pub acknowledged_by: Option<String>,
    pub closed_by: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub id: String,
    pub tiny_id: String,
    pub message: String,
    pub status: String,
    pub acknowledged: bool,
    pub is_seen: bool,
    pub tags: Option<Vec<String>>,
    pub snoozed: bool,
    pub snoozed_until: Option<DateTime<FixedOffset>>,
    pub count: u64,
    pub last_occurred_at: DateTime<FixedOffset>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub source: String,
    pub owner: Option<String>,
    pub priority: String,
    pub responders: Vec<Responder>,
    pub report: Option<Report>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::response_test;

    #[test]
    fn count_response() {
        let fixture = r#"
        {
    "data": {
        "count": 7
    },
    "took": 0.051,
    "requestId": "9ae63dd7-ed00-4c81-86f0-c4ffd33142c9"
}
"#;
        response_test::<Count>(fixture);
    }

    #[test]
    fn list_resposne() {
        let fixture = r#"
        {
    "data": [
        {
            "id": "70413a06-38d6-4c85-92b8-5ebc900d42e2",
            "tinyId": "1791",
            "alias": "event_573",
            "message": "Our servers are in danger",
            "status": "closed",
            "acknowledged": false,
            "isSeen": true,
            "tags": [
                "OverwriteQuietHours",
                "Critical"
            ],
            "snoozed": true,
            "snoozedUntil": "2017-04-03T20:32:35.143Z",
            "count": 79,
            "lastOccurredAt": "2017-04-03T20:05:50.894Z",
            "createdAt": "2017-03-21T20:32:52.353Z",
            "updatedAt": "2017-04-03T20:32:57.301Z",
            "source": "Isengard",
            "owner": "morpheus@opsgenie.com",
            "priority": "P4",
            "responders":[
              {
                  "id":"4513b7ea-3b91-438f-b7e4-e3e54af9147c",
                  "type":"team"
              },
              {
                  "id":"bb4d9938-c3c2-455d-aaab-727aa701c0d8",
                  "type":"user"
              },
              {
                  "id":"aee8a0de-c80f-4515-a232-501c0bc9d715",
                  "type":"escalation"
              },
              {
                  "id":"80564037-1984-4f38-b98e-8a1f662df552",
                  "type":"schedule"
              }
            ],
            "integration": {
                "id": "4513b7ea-3b91-438f-b7e4-e3e54af9147c",
                "name": "Nebuchadnezzar",
                "type": "API"
            },
            "report": {
                "ackTime": 15702,
                "closeTime": 60503,
                "acknowledgedBy": "agent_smith@opsgenie.com",
                "closedBy": "neo@opsgenie.com"
            }
        },
        {
            "id": "70413a06-38d6-4c85-92b8-5ebc900d42e2",
            "tinyId": "1791",
            "alias": "event_573",
            "message": "Sample Message",
            "status": "open",
            "acknowledged": false,
            "isSeen": false,
            "tags": [
                "RandomTag"
            ],
            "snoozed": false,
            "count": 1,
            "lastOccurredAt": "2017-03-21T20:32:52.353Z",
            "createdAt": "2017-03-21T20:32:52.353Z",
            "updatedAt": "2017-04-03T20:32:57.301Z",
            "source": "Zion",
            "owner": "",
            "priority": "P5",
            "responders":[],
            "integration": {
                "id": "4513b7ea-3b91-b7e4-438f-e3e54af9147c",
                "name": "My_Lovely_Amazon",
                "type": "CloudWatch"
            }
        }

    ],
    "paging":{
        "next":"https://api.opsgenie.com/v2/alerts?query=status%3Aopen&offset=20&limit=10&sort=createdAt&order=desc",
        "first":"https://api.opsgenie.com/v2/alerts?query=status%3Aopen&offset=0&limit=10&sort=createdAt&order=desc",
        "last":"https://api.opsgenie.com/v2/alerts?query=status%3Aopen&offset=100&limit=10&sort=createdAt&order=desc"
    },
    "took": 0.605,
    "requestId": "9ae63dd7-ed00-4c81-86f0-c4ffd33142c9"
}
"#;
        response_test::<Vec<Alert>>(fixture);
    }
}
