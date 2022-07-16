use crate::gecko;
use crate::types::Response;
use serde_json;

pub enum Order {
    MarketCapDesc,
    MarketCapAsc,
    NameDesc,
    NameAsc,
    MarketCapChange24hDesc,
    MarketCapChange24hAsc,
}

impl Order {
    fn as_str(&self) -> &'static str {
        match self {
            Order::MarketCapDesc => "market_cap_desc",
            Order::MarketCapAsc => "market_cap_asc",
            Order::NameDesc => "name_desc",
            Order::NameAsc => "name_asc",
            Order::MarketCapChange24hDesc => "market_cap_change_24h_desc",
            Order::MarketCapChange24hAsc => "market_cap_change_24h_asc",
        }
    }
}

pub fn get(order: Option<Order>) -> Response<serde_json::Value> {
    let mut params = "?".to_string();
    if !order.is_none() {
        params.push_str(&["order", order.unwrap().as_str()].join("="));
    }
    let response = gecko::get_request("/coins/categories", &params);
    response
}

pub fn list() -> Response<serde_json::Value> {
    let response = gecko::get_request("/coins/categories/list", "");
    response
}
