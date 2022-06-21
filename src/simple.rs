use crate::gecko;
use crate::gecko::vec_str_2_comma_str;
use crate::types::Response;
use std::collections::HashMap;

/// Get the current price of any cryptocurrencies in any other supported currencies that you need.
pub fn price(
    ids: Vec<&str>,
    vs_currencies: Vec<&str>,
    include_market_cap: Option<bool>,
    include_24hr_vol: Option<bool>,
    include_24hr_change: Option<bool>,
    include_last_updated_at: Option<bool>,
) -> Response<HashMap<String, HashMap<String, f32>>> {
    let endpoint = "/simple/price?".to_string();
    let ids_str: String = ["ids", &vec_str_2_comma_str(ids)].join("=");
    let vs_currencies_str: String =
        ["vs_currencies", &vec_str_2_comma_str(vs_currencies)].join("=");

    let mut params = [ids_str, vs_currencies_str].join("&");

    let imc = include_market_cap.unwrap_or(false);
    let i24v = include_24hr_vol.unwrap_or(false);
    let i24c = include_24hr_change.unwrap_or(false);
    let ilu = include_last_updated_at.unwrap_or(false);

    if imc {
        params.push_str("&include_market_cap=true");
    }
    if i24v {
        params.push_str("&include_24hr_vol=true");
    }

    if i24c {
        params.push_str("&include_24hr_change=true");
    }

    if ilu {
        params.push_str("&include_last_updated_at=true");
    }

    let response = gecko::get_request(&endpoint, &params);
    response
}

/// Get current price of tokens (using contract addresses) for a given platform in any other currency that you need.
pub fn token_price(
    id: &str,
    contract_addresses: Vec<&str>,
    vs_currencies: Vec<&str>,
    include_market_cap: Option<bool>,
    include_24hr_vol: Option<bool>,
    include_24hr_change: Option<bool>,
    include_last_updated_at: Option<bool>,
) -> Response<HashMap<String, f32>> {
    let endpoint = ["/simple/token_price/", id, "?"].join("");

    let contract_addresses: String = [
        "contract_addresses",
        &vec_str_2_comma_str(contract_addresses),
    ]
    .join("=");
    let vs_currencies_str: String =
        ["vs_currencies", &vec_str_2_comma_str(vs_currencies)].join("=");

    let mut params = [contract_addresses, vs_currencies_str].join("&");

    let imc = include_market_cap.unwrap_or(false);
    let i24v = include_24hr_vol.unwrap_or(false);
    let i24c = include_24hr_change.unwrap_or(false);
    let ilu = include_last_updated_at.unwrap_or(false);

    if imc {
        params.push_str("&include_market_cap=true");
    }
    if i24v {
        params.push_str("&include_24hr_vol=true");
    }

    if i24c {
        params.push_str("&include_24hr_change=true");
    }

    if ilu {
        params.push_str("&include_last_updated_at=true");
    }

    let response = gecko::get_request(&endpoint, &params);
    response
}

/// Get list of supported_vs_currencies.
pub fn supported_vs_currencies() -> Response<Vec<String>> {
    let endpoint = "/simple/supported_vs_currencies";
    let response = gecko::get_request(&endpoint, "");
    response
}
