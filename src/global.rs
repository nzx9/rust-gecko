use crate::gecko;
use crate::types::Response;
use serde_json;

pub fn data() -> Response<serde_json::Value> {
    let response = gecko::get_request("/global", "");
    response
}

pub fn decentralized_finance_defi() -> Response<serde_json::Value> {
    let response = gecko::get_request("/global/decentralized_finance_defi", "");
    response
}
