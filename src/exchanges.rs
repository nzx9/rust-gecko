use crate::gecko;
use crate::types::Response;
use serde_json;

pub enum Depth {
    CostToMoveUpUsd,
    CostToMoveDownUsd,
}

impl Depth {
    pub fn as_str(&self) -> &'static str {
        match self {
            Depth::CostToMoveUpUsd => "cost_to_move_up_usd",
            Depth::CostToMoveDownUsd => "cost_to_move_down_usd",
        }
    }
}

pub enum Order {
    TrustScoreDesc,
    TrustScoreAsc,
    VolumeDesc,
}

impl Order {
    pub fn as_str(&self) -> &'static str {
        match self {
            Order::TrustScoreDesc => "trust_score_desc",
            Order::TrustScoreAsc => "trust_score_asc",
            Order::VolumeDesc => "volume_desc",
        }
    }
}

pub fn get(per_page: Option<usize>, page: Option<usize>) -> Response<serde_json::Value> {
    let url = String::from("/exchanges");
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
pub fn list() -> Response<serde_json::Value> {
    let response = gecko::get_request("/exchanges/list", "");
    response
}

pub fn by_id(id: &str) -> Response<serde_json::Value> {
    let url = String::from(["/exchanges", id].join("="));
    let response = gecko::get_request(&url, "");
    response
}

pub fn tickers(
    id: &str,
    coin_ids: Option<Vec<&str>>,
    include_exchange_logo: Option<bool>,
    page: Option<usize>,
    depth: Option<Depth>,
    order: Option<Order>,
) -> Response<serde_json::Value> {
    let url = String::from(["/exchanges/", id, "/tickers"].join("/"));
    let mut params = String::from("?");

    params = gecko::append_if(
        &mut params,
        !coin_ids.is_none(),
        Some(&["coin_ids", &gecko::vec_str_2_comma_str(coin_ids.unwrap())].join("=")),
        None,
    );

    params = gecko::append_if(
        &mut params,
        !include_exchange_logo.is_none(),
        Some(
            &[
                "include_exchange_logo",
                &include_exchange_logo.unwrap().to_string(),
            ]
            .join("="),
        ),
        None,
    );

    params = gecko::append_if(
        &mut params,
        !page.is_none(),
        Some(&["page", &page.unwrap().to_string()].join("=")),
        None,
    );

    params = gecko::append_if(
        &mut params,
        !depth.is_none(),
        Some(&["depth", &depth.unwrap().as_str()].join("=")),
        None,
    );

    params = gecko::append_if(
        &mut params,
        !order.is_none(),
        Some(&["order", &order.unwrap().as_str()].join("=")),
        None,
    );

    let response = gecko::get_request(&url, &params);
    response
}

pub fn volume_chart(id: &str, days: usize) -> Response<serde_json::Value> {
    let url = String::from(["/exchanges/", id, "/volume_chart"].join("/"));
    let params = format!("?days={}", days);
    let response = gecko::get_request(&url, &params);
    response
}
