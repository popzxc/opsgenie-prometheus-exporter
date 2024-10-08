use serde::Deserialize;

use crate::api::team::response as team_response;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub id: String,
    pub name: String,
    pub owner_team: team_response::Team,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::response_test;

    #[test]
    fn list_schedules_response() {
        let fixture = r#"{
    "data": [
        {
            "id": "d875e654-9b4e-4219-alp3-0c26936d18de",
            "name": "ScheduleName",
            "description": "ScheduleDescription",
            "timezone": "Europe/Kirov",
            "enabled": true,
            "ownerTeam": {
                "id": "90098alp-f0e3-41d3-a060-0ea895027630",
                "name": "ops_team"
            },
            "rotations": [
                {
                    "id": "a47alp93-0541-4aa3-bac6-4084cfa02d20",
                    "name": "First Rotation",
                    "startDate": "2017-05-14T21:00:00Z",
                    "type": "weekly",
                    "length": 1,
                    "participants": [
                        {
                            "type": "user",
                            "id": "a9514028-2bca-4510-alpf-4b65f2c33a56",
                            "username": "user@opsgenie.com"
                        },
                        {
                            "type": "team",
                            "id": "00564944-b42f-4b95-a882-ee9a5alpb9bb",
                            "name": "ops_team"
                        }
                    ]
                }
            ]
        }
    ],
    "expandable": [
        "rotation"
    ],
    "took": 0.096,
    "requestId": "663alpfc-e647-4759-8121-7d33e34c01c1"
}
        "#;

        response_test::<Vec<Schedule>>(fixture);
    }
}
