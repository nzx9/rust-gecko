use crate::gecko;
use crate::types::Response;
use serde_json;
pub enum CoinId {
    Bitcoin,
    Ethereum,
}

impl CoinId {
    fn as_str(&self) -> &'static str {
        match self {
            CoinId::Bitcoin => "bitcoin",
            CoinId::Ethereum => "ethereum",
        }
    }
}

pub fn get(coin_id: CoinId) -> Response<serde_json::Value> {
    let url = format!("/companies/public_treasury/{}", coin_id.as_str());

    let response = gecko::get_request(&url, "");
    response
}
