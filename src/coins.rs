use crate::gecko;
use crate::types::Response;
use serde_json;
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
pub enum TickersOrder {
    TrustScoreDesc,
    TrustScoreAsc,
    VolumeDesc,
}

impl TickersOrder {
    fn as_str(&self) -> &'static str {
        match self {
            TickersOrder::TrustScoreDesc => "trust_score_desc",
            TickersOrder::TrustScoreAsc => "trust_score_asc",
            TickersOrder::VolumeDesc => "volume_desc",
        }
    }
}

pub fn list(include_platform: Option<bool>) -> Response<serde_json::Value> {
    let params = gecko::append_if(
        &mut String::from("?"),
        !include_platform.is_none(),
        Some(&["include_platform", &include_platform.unwrap().to_string()].join("=")),
        None,
    );
    let response = gecko::get_request("/coins/list", &params);
    response
}

pub fn markets(
    vs_currency: &str,
    ids: Option<Vec<&str>>,
    category: Option<&str>,
    order: Option<Order>,
) -> Response<serde_json::Value> {
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

    let response = gecko::get_request("/coins/markets", &params);
    response
}

/// Get current data (name, price, market, ... including exchange tickers) for a coin.
pub fn get(
    id: &str,
    localization: Option<bool>,
    tickers: Option<bool>,
    market_data: Option<bool>,
    community_data: Option<bool>,
    developer_data: Option<bool>,
    sparkline: Option<bool>,
) -> Response<serde_json::Value> {
    let loc = localization.unwrap_or(true);
    let tik = tickers.unwrap_or(true);
    let m_d = market_data.unwrap_or(true);
    let c_d = community_data.unwrap_or(true);
    let d_d = developer_data.unwrap_or(true);
    let spk = sparkline.unwrap_or(false);

    let mut params = gecko::append_if(
        "?",
        loc,
        Some("localization=true"),
        Some("localization=false"),
    );
    params = gecko::append_if(&params, tik, Some("tickers=true"), Some("tickers=false"));
    params = gecko::append_if(
        &params,
        m_d,
        Some("market_data=true"),
        Some("market_data=false"),
    );
    params = gecko::append_if(
        &params,
        c_d,
        Some("community_data=true"),
        Some("community_data=false"),
    );
    params = gecko::append_if(
        &params,
        d_d,
        Some("developer_data=true"),
        Some("developer_data=false"),
    );
    params = gecko::append_if(
        &params,
        spk,
        Some("sparkline=true"),
        Some("sparkline=false"),
    );

    let response = gecko::get_request(&["/coins", id].join("/"), &params);
    response
}

/// Get coin tickers
pub fn tickers(
    id: &str,
    exchange_ids: Option<Vec<&str>>,
    include_exchange_logo: Option<bool>,
    page: Option<u16>,
    order: Option<TickersOrder>,
    depth: Option<bool>,
) -> Response<serde_json::Value> {
    let mut params = gecko::append_if(
        "?",
        !exchange_ids.is_none(),
        Some(&gecko::vec_str_2_comma_str(
            exchange_ids.unwrap_or(vec![""]),
        )),
        None,
    );

    params = gecko::append_if(
        &params,
        include_exchange_logo.unwrap_or(false),
        Some(&"include_exchange_logo"),
        None,
    );

    params = gecko::append_if(
        &params,
        !page.is_none(),
        Some(&["page", &page.unwrap_or(0).to_string()].join("=")),
        None,
    );

    params = gecko::append_if(
        &params,
        !order.is_none(),
        Some(
            &[
                "order",
                order.unwrap_or(TickersOrder::TrustScoreDesc).as_str(),
            ]
            .join("="),
        ),
        None,
    );
    params = gecko::append_if(&params, depth.unwrap_or(false), Some("depth=true"), None);

    let response = gecko::get_request(&["/coins", id, "tickers"].join("/"), &params);
    response
}

pub fn history(id: &str, date: &str, localization: Option<bool>) -> Response<serde_json::Value> {
    let mut params = ["?date", date].join("=");
    params = gecko::append_if(
        &params,
        localization.unwrap_or(false),
        Some("&localization"),
        None,
    );
    let response = gecko::get_request(&["/coins", id, "history"].join("/"), &params);
    response
}
pub fn market_chart(
    id: &str,
    vs_currency: &str,
    days: &str,
    interval: Option<&str>,
) -> Response<serde_json::Value> {
    let mut params = ["?vs_currency=", vs_currency, "&days=", days].join("");
    if !interval.is_none() {
        params.push_str(&["&interval", interval.unwrap()].join("="));
    }
    let response = gecko::get_request(&["/coins", id, "market_chart"].join("/"), &params);
    response
}

pub fn market_chart_range(
    id: &str,
    vs_currency: &str,
    from: &str,
    to: &str,
) -> Response<serde_json::Value> {
    let response = gecko::get_request(
        &["/coins", id, "market_chart", "range"].join("/"),
        &["?vs_currency", vs_currency, "&from", from, "&to", to].join("="),
    );
    response
}

pub fn ohlc(id: &str, vs_currency: &str, days: &str) -> Response<serde_json::Value> {
    let response = gecko::get_request(
        &["/coins", id, "ohlc"].join("/"),
        &["?vs_currency", vs_currency, "&days", days].join("="),
    );
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_chart() {
        let response = market_chart("bitcoin", "usd", "1", Some("daily"));
        assert!(response.is_success);
    }
}
