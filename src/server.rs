use crate::gecko;
use crate::types::Response;
use std::collections::HashMap;

/// Check API server status
pub fn ping() -> Response<HashMap<String, String>> {
    let endpoint = "/ping".to_string();
    let response = gecko::get_request(&endpoint, "");
    response
}
