/// Get the current price of any cryptocurrencies in any other supported currencies that you need.
///
/// Equivalent to the [`crate::simple::price`] function.
///
/// # Examples
///
/// ```
/// rust_gecko::simple_price!(vec!["bitcoin", "litecoin"], vec!["usd", "lkr"]);
/// ```
#[macro_export]
macro_rules! simple_price {
    ($ids: expr,
        $vs_currencies: expr) => {
        crate::rust_gecko::simple::price($ids, $vs_currencies, None, None, None, None)
    };
    ($ids: expr,
        $vs_currencies: expr,
        $include_market_cap:expr,
        $include_24hr_vol: expr,) => {
        crate::rust_gecko::simple::price(
            $ids,
            $vs_currencies,
            $include_market_cap,
            None,
            None,
            None,
        );
    };
    ($ids: expr,
        $vs_currencies: expr,
        $include_market_cap:expr,
        $include_24hr_vol: expr,
        ) => {
        crate::rust_gecko::simple::price(
            $ids,
            $vs_currencies,
            $include_market_cap,
            $include_24hr_vol,
            None,
            None,
        )
    };
    ($ids: expr,
        $vs_currencies: expr,
        $include_market_cap:expr,
        $include_24hr_vol: expr,
        $include_24hr_change: expr) => {
        crate::rust_gecko::simple::price(
            $ids,
            $vs_currencies,
            $include_market_cap,
            $include_24hr_vol,
            $include_24hr_change,
            None,
        )
    };
    ($ids: expr,
        $vs_currencies: expr,
        $include_market_cap:expr,
        $include_24hr_vol: expr,
        $include_24hr_change: expr,
        $include_last_updated_at: expr) => {
        crate::rust_gecko::simple::price(
            $ids,
            $vs_currencies,
            $include_market_cap,
            $include_24hr_vol,
            $include_24hr_change,
            $include_last_updated_at,
        )
    };
}

/// Get current price of tokens (using contract addresses) for a given platform in any other currency that you need.
///
/// Equivalent to the [`crate::simple::token_price`] function.
///
/// # Examples
///
/// ```
/// rust_gecko::simple_token_price!("bitcoin", vec!["0x8ff795a6f4d97e7887c79bea79aba5cc76444adf"], vec!["usd", "lkr"]);
/// ```
#[macro_export]
macro_rules! simple_token_price {
    ($id: expr,
        $contract_addresses: expr,
        $vs_currencies: expr,) => {
        crate::rust_gecko::simple::token_price(
            $id,
            $contract_addresses,
            $vs_currencies,
            None,
            None,
            None,
            None,
        )
    };
    ($id: expr,
        $contract_addresses: expr,
        $vs_currencies: expr,
        $include_market_cap:expr) => {
        crate::rust_gecko::simple::token_price(
            $id,
            $contract_addresses,
            $vs_currencies,
            $include_market_cap,
            None,
            None,
            None,
        )
    };
    ($id: expr,
        $contract_addresses: expr,
        $vs_currencies: expr,
        $include_market_cap:expr,
        $include_24hr_vol: expr) => {
        crate::rust_gecko::simple::token_price(
            $id,
            $contract_addresses,
            $vs_currencies,
            $include_market_cap,
            $include_24hr_vol,
            None,
            None,
        )
    };
    ($id: expr,
        $contract_addresses: expr,
        $vs_currencies: expr,
        $include_market_cap:expr,
        $include_24hr_vol: expr,
        $include_24hr_change: expr) => {
        crate::rust_gecko::simple::token_price(
            $id,
            $contract_addresses,
            $vs_currencies,
            $include_market_cap,
            $include_24hr_vol,
            $include_24hr_change,
            None,
        )
    };
    ($id: expr,
        $contract_addresses: expr,
        $vs_currencies: expr,
        $include_market_cap:expr,
        $include_24hr_vol: expr,
        $include_24hr_change: expr,
        $include_last_updated_at: expr) => {
        crate::rust_gecko::simple::token_price(
            $id,
            $contract_addresses,
            $vs_currencies,
            $include_market_cap,
            $include_24hr_vol,
            $include_24hr_change,
            $include_last_updated_at,
        )
    };
}
