use serde::{Deserialize, Serialize};

use crate::gecko;
use std::collections::HashMap;

pub enum Order {
    GeckoDesc,
    GeckoAsc,
    MarketCapDesc,
    MarketCapAsc,
    VolumeDesc,
    VolumeAsc,
    IdDesc,
    IdAsc,
}

impl Order {
    fn as_str(&self) -> &'static str {
        match self {
            Order::GeckoDesc => "gecko_desc",
            Order::GeckoAsc => "gecko_asc",
            Order::MarketCapDesc => "market_cap_desc",
            Order::MarketCapAsc => "market_cap_asc",
            Order::VolumeDesc => "volume_desc",
            Order::VolumeAsc => "volume_asc",
            Order::IdDesc => "id_desc",
            Order::IdAsc => "id_asc",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListWithPlatformType {
    id: String,
    symbol: String,
    name: String,
    platforms: HashMap<Option<String>, Option<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListWithoutPlatformType {
    id: String,
    symbol: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoiType {
    times: f64,
    currency: String,
    percentage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketType {
    id: String,
    symbol: String,
    name: String,
    image: String,
    current_price: f64,
    market_cap: f64,
    market_cap_rank: Option<u32>,
    fully_diluted_valuation: Option<f64>,
    total_volume: Option<f64>,
    high_24h: f64,
    low_24h: f64,
    price_change_24h: f64,
    price_change_percentage_24h: f64,
    market_cap_change_24h: f64,
    market_cap_change_percentage_24h: f64,
    circulating_supply: f64,
    total_supply: Option<f64>,
    max_supply: Option<f64>,
    ath: f64,
    ath_change_percentage: f64,
    ath_date: String,
    atl: f64,
    atl_change_percentage: f64,
    atl_date: String,
    roi: Option<RoiType>,
    last_updated: String,
}

pub fn list() -> (reqwest::StatusCode, Vec<ListWithoutPlatformType>) {
    let (status, resp_json) = gecko::get_request("/coins/list", "");
    (status, resp_json)
}

pub fn list_with_platform() -> (reqwest::StatusCode, Vec<ListWithPlatformType>) {
    let (status, resp_json) = gecko::get_request("/coins/list", "?include_platform=true");
    (status, resp_json)
}

pub fn markets(
    vs_currency: &str,
    ids: Option<Vec<&str>>,
    category: Option<&str>,
    order: Option<Order>,
) -> (reqwest::StatusCode, Vec<MarketType>) {
    let mut params = ["?vs_currency", vs_currency].join("=");

    if !ids.is_none() {
        params.push_str(&["&ids", &gecko::vec_str_2_comma_str(ids.unwrap())].join("="));
    }

    if !category.is_none() {
        params.push_str(&["&category", &category.unwrap()].join("="));
    }

    if !order.is_none() {
        params.push_str(&["&order", &order.unwrap().as_str()].join("="));
    }

    let (status, resp_json) = gecko::get_request("/coins/markets", &params);
    (status, resp_json)
}

/// Get current data (name, price, market, ... including exchange tickers) for a coin.
pub fn coin() {}

/// Get coin tickers
pub fn tickers() {}

pub fn history() {}

pub fn market_chart() {}

pub fn market_chart_range() {}

pub fn ohlc() {}
