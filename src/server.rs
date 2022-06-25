use crate::gecko;
use crate::types::Response;
use serde_json;

/// Check API server status
pub fn ping() -> Response<serde_json::Value> {
    let endpoint = "/ping".to_string();
    let response = gecko::get_request(&endpoint, "");
    response
}
