use crate::gecko;
use crate::types::Response;
use serde_json;

pub fn query(query: &str) -> Response<serde_json::Value> {
    let response = gecko::get_request("/search?query=", query);
    response
}

pub fn trending() -> Response<serde_json::Value> {
    let response = gecko::get_request("/search/trending", "");
    response
}
