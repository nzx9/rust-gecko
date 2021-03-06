use crate::gecko;
use crate::types::Response;
use serde_json;

pub fn get(query: &str) -> Response<serde_json::Value> {
    let response = gecko::get_request("/search?query=", query);
    response
}
