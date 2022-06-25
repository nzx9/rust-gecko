use crate::gecko;
use crate::types::Response;
use serde_json;
pub fn rates() -> Response<serde_json::Value> {
    let response = gecko::get_request("/exchange_rates", "");
    response
}
