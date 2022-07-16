use crate::gecko;
use crate::types::Response;
use serde_json;

pub fn get(per_page: Option<usize>, page: Option<usize>) -> Response<serde_json::Value> {
    let url = String::from("/indexes");
    let mut params = String::from("?");

    params = gecko::append_if(
        &mut params,
        !per_page.is_none(),
        Some(&["per_page", &per_page.unwrap().to_string()].join("=")),
        None,
    );

    params = gecko::append_if(
        &mut params,
        !page.is_none(),
        Some(&["page", &page.unwrap().to_string()].join("=")),
        None,
    );

    let response = gecko::get_request(&url, &params);
    response
}

pub fn indexes(market_id: &str, id: &str) -> Response<serde_json::Value> {
    let url = String::from(["/indexes", market_id, id].join("="));

    let response = gecko::get_request(&url, "");
    response
}

pub fn list() -> Response<serde_json::Value> {
    let response = gecko::get_request("/indexes/list", "");
    response
}
