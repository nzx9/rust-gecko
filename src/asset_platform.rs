use crate::gecko;
use crate::types::Response;
use serde_json;

pub fn get() -> Response<serde_json::Value> {
    let response = gecko::get_request("/asset_platform", "");
    response
}
