use crate::gecko;
use crate::gecko::vec_str_2_comma_str;
use std::collections::HashMap;

static mut IMC: bool = false;
static mut I24V: bool = false;
static mut I24C: bool = false;
static mut ILU: bool = false;

pub mod market_cap {
    use crate::simple::IMC;
    /// Include market cap in the response
    pub fn include() -> () {
        unsafe {
            IMC = true;
        }
    }
    /// Exclude market cap in the response
    pub fn exclude() -> () {
        unsafe {
            IMC = false;
        }
    }
}

pub mod vol_24hr {
    use crate::simple::I24V;
    /// Include volume 24hr in the response
    pub fn include() -> () {
        unsafe {
            I24V = true;
        }
    }
    /// Exclude volume 24hr in the response
    pub fn exclude() -> () {
        unsafe {
            I24V = false;
        }
    }
}

pub mod chng_24hr {
    use crate::simple::I24C;
    /// Include change in 24hr in the response
    pub fn include() -> () {
        unsafe {
            I24C = true;
        }
    }
    /// Exclude change in 24hr in the response
    pub fn exclude() -> () {
        unsafe {
            I24C = false;
        }
    }
}

pub mod last_updated {
    use crate::simple::ILU;
    /// Include last updated at in the response
    pub fn include() -> () {
        unsafe {
            ILU = true;
        }
    }
    /// Exclude last updated at in the response
    pub fn exclude() -> () {
        unsafe {
            ILU = false;
        }
    }
}

/// Get the current price of any cryptocurrencies in any other supported currencies that you need.
pub fn price(
    ids: Vec<&str>,
    vs_currencies: Vec<&str>,
) -> (reqwest::StatusCode, HashMap<String, HashMap<String, f32>>) {
    let endpoint = "/simple/price?".to_string();

    let ids_str: String = ["ids", &vec_str_2_comma_str(ids)].join("=");
    let vs_currencies_str: String =
        ["vs_currencies", &vec_str_2_comma_str(vs_currencies)].join("=");

    let mut params = [ids_str, vs_currencies_str].join("&");

    unsafe {
        if IMC {
            params.push_str("&include_market_cap=true");
        }
        if I24V {
            params.push_str("&include_24hr_vol=true");
        }

        if I24C {
            params.push_str("&include_24hr_change=true");
        }

        if ILU {
            params.push_str("&include_last_updated_at=true");
        }
    }

    let (status, resp_json) = gecko::get_request(&endpoint, &params);
    (status, resp_json)
}

/// Get current price of tokens (using contract addresses) for a given platform in any other currency that you need.
pub fn token_price(
    id: &str,
    contract_addresses: Vec<&str>,
    vs_currencies: Vec<&str>,
) -> (reqwest::StatusCode, HashMap<String, f32>) {
    let endpoint = ["/simple/token_price/", id, "?"].join("");

    let contract_addresses: String = [
        "contract_addresses",
        &vec_str_2_comma_str(contract_addresses),
    ]
    .join("=");
    let vs_currencies_str: String =
        ["vs_currencies", &vec_str_2_comma_str(vs_currencies)].join("=");

    let mut params = [contract_addresses, vs_currencies_str].join("&");

    unsafe {
        if IMC {
            params.push_str("&include_market_cap=true");
        }
        if I24V {
            params.push_str("&include_24hr_vol=true");
        }

        if I24C {
            params.push_str("&include_24hr_change=true");
        }

        if ILU {
            params.push_str("&include_last_updated_at=true");
        }
    }

    let (status, resp_json) = gecko::get_request(&endpoint, &params);
    (status, resp_json)
}

/// Get list of supported_vs_currencies.
pub fn supported_vs_currencies() -> (reqwest::StatusCode, Vec<String>) {
    let endpoint = "/simple/supported_vs_currencies";
    let (status, resp_json) = gecko::get_request(&endpoint, "");
    (status, resp_json)
}
