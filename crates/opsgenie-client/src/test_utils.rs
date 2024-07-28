use serde::de::DeserializeOwned;

use crate::api::response::ApiResponse;

/// Checks whether the `ApiResponse<T>`` can be deserialized.
pub fn response_test<T: DeserializeOwned>(fixture: &str) {
    let _response: ApiResponse<T> = serde_json::from_str(fixture).expect("Unable to deserialize");
}
