use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamMember {
    pub user: User,
    pub role: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamDescriptor {
    pub id: String,
    pub description: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: String,
    pub description: Option<String>,
    pub name: String,
    pub members: Option<Vec<TeamMember>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::response_test;

    #[test]
    fn get_team_response() {
        let fixture = r#"{
    "data": {
        "id": "a30alp45-65bf-422f-9d41-67b10a67282a",
        "name": "TeamName",
        "description": "Team Description",
        "members": [
            {
                "user": {
                    "id": "a9514028-2bca-4510-a51f-4b65f2c33alp",
                    "username": "user@opsgenie.com"
                },
                "role": "admin"
            },
            {
                "user": {
                    "id": "00564944-b42f-4b95-a882-ee9a5aalp9bb",
                    "username": "user2@opsgenie.com"
                },
                "role": "user"
            },
            {
                "user": {
                    "id": "1f281991-bca3-4ae2-bdea-b02e94dalp53",
                    "username": "user3@opsgenie.com"
                },
                "role": "user"
            }
        ]
    },
    "took": 0.021,
    "requestId": "36d5c8c5-alpf-47b2-9964-9fd435e5e306"
}
"#;
        response_test::<Team>(fixture);
    }

    #[test]
    fn list_teams_response() {
        let fixture = r#"
        {
    "data": [
        {
            "id": "90098alp9-f0e3-41d3-a060-0ea895027630",
            "name": "ops_team",
            "description": ""
        },
        {
            "id": "a30alp45-65bf-422f-9d41-67b10a67282a",
            "name": "TeamName2",
            "description": "Description"
        },
        {
            "id": "c569c016-alp9-4e20-8a28-bd5dc33b798e",
            "name": "TeamName",
            "description": ""
        }
    ],
    "took": 1.08,
    "requestId": "9cbfalp7-53f5-41ef-a360-be01277a903d"
}
"#;
        response_test::<Vec<Team>>(fixture);
    }
}
