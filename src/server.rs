use crate::gecko;
use std::collections::HashMap;

/// Check API server status
pub fn ping() -> (reqwest::StatusCode, HashMap<String, String>) {
    let (status, resp_json) = gecko::get_request("/ping", "");
    (status, resp_json)
}
