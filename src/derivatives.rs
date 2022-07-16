use crate::gecko;
use crate::types::Response;
use serde_json;

pub enum Order {
    NameAsc,
    NameDesc,
    OpenInterestBtcAsc,
    OpenInterestBtcDesc,
    TradeVolume24hBtcAsc,
    TradeVolume24hBtcDesc,
}

impl Order {
    fn as_str(&self) -> &'static str {
        match self {
            Order::NameAsc => "name_asc",
            Order::NameDesc => "name_desc",
            Order::OpenInterestBtcAsc => "open_interest_btc_asc",
            Order::OpenInterestBtcDesc => "open_interest_btc_desc",
            Order::TradeVolume24hBtcAsc => "trade_volume_24h_btc_asc",
            Order::TradeVolume24hBtcDesc => "trade_volume_24h_btc_desc",
        }
    }
}

pub enum Tickers {
    All,
    Unexpired,
}

impl Tickers {
    fn as_str(&self) -> &'static str {
        match self {
            Tickers::All => "all",
            Tickers::Unexpired => "unexpired",
        }
    }
}

pub fn get(include_tickers: Option<Tickers>) -> Response<serde_json::Value> {
    let url = String::from("/tickers");
    let mut params = String::from("?");

    params = gecko::append_if(
        &mut params,
        !include_tickers.is_none(),
        Some(&["include_tickers", &include_tickers.unwrap().as_str()].join("=")),
        None,
    );

    let response = gecko::get_request(&url, &params);
    response
}

pub fn exchanges(
    order: Option<Order>,
    per_page: Option<usize>,
    page: Option<usize>,
) -> Response<serde_json::Value> {
    let url = String::from("/exchanges");

    let mut params = String::from("?");

    params = gecko::append_if(
        &mut params,
        !order.is_none(),
        Some(&["order", &order.unwrap().as_str()].join("=")),
        None,
    );

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

pub fn exchanges_id(id: &str, include_tickers: Option<Tickers>) -> Response<serde_json::Value> {
    let mut url = String::from("/exchanges/");
    url.push_str(id);
    let mut params = String::from("?");

    params = gecko::append_if(
        &mut params,
        !include_tickers.is_none(),
        Some(&["include_tickers", &include_tickers.unwrap().as_str()].join("=")),
        None,
    );
    let response = gecko::get_request(&url, &params);
    response
}

pub fn exchanges_list() -> Response<serde_json::Value> {
    let response = gecko::get_request("/exchanges/list", "");
    response
}
