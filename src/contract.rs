use crate::gecko;
use crate::types::Response;
use serde_json;

pub fn get(id: &str, contract_address: &str) -> Response<serde_json::Value> {
    let url = format!("coins/{}/contract/{}", id, contract_address);
    let response = gecko::get_request(&url, "");
    response
}

pub fn market_chart(
    id: &str,
    contract_address: &str,
    vs_currency: &str,
    days: usize,
) -> Response<serde_json::Value> {
    let url = format!("coins/{}/contract/{}/market_chart", id, contract_address);
    let params = format!("vs_currency={}&days={}", vs_currency, days);
    let response = gecko::get_request(&url, &params);
    response
}

pub fn market_chart_range(
    id: &str,
    contract_address: &str,
    vs_currency: &str,
    from: &str,
    to: &str,
) -> Response<serde_json::Value> {
    let url = format!(
        "coins/{}/contract/{}/market_chart_range",
        id, contract_address
    );
    let params = format!("vs_currency={}&from={}&to={}", vs_currency, from, to);
    let response = gecko::get_request(&url, &params);
    response
}
