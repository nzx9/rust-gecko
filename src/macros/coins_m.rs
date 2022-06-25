/// List all supported coins price, market cap, volume, and market related data.
///
/// Equivalent to the [`crate::coins::markets`] function.
///
/// # Examples
///
/// ```
/// rust_gecko::coins_markets!("usd");
/// rust_gecko::coins_markets!("usd", Some(vec!["bitcoin", "litecoin"]));
/// ```
#[macro_export]
macro_rules! coins_markets {
    ($vs_currency: expr) => {
        crate::rust_gecko::coins::markets($vs_currency, None, None, None)
    };
    ($vs_currency: expr, $ids: expr) => {
        crate::rust_gecko::coins::markets($vs_currency, $ids, None, None)
    };
    ($vs_currency: expr, $ids: expr, $category: expr) => {
        crate::rust_gecko::coins::markets($vs_currency, $ids, $category, None)
    };
    ($vs_currency: expr, $ids: expr, $category: expr, $order: expr) => {
        crate::rust_gecko::coins::markets($vs_currency, $ids, $category, $order)
    };
}

/// Get coin tickers (paginated to 100 items)
///
/// Equivalent to the [`crate::coins::tickers`] function.
///
/// # Examples
///
/// ```
/// rust_gecko::coins_tickers!("bitcoin");
/// ```
#[macro_export]
macro_rules! coins_tickers {
    ($id: expr) => {
        crate::rust_gecko::coins::tickers($id, None, None, None, None, None)
    };
    ($id: expr,
    $exchange_ids: expr) => {
        crate::rust_gecko::coins::tickers($id, $exchange_ids, None, None, None, None)
    };
    ($id: expr,
    $exchange_ids: expr,
    $include_exchange_logo: expr) => {
        crate::rust_gecko::coins::tickers(
            $id,
            $exchange_ids,
            $include_exchange_logo,
            None,
            None,
            None,
        )
    };
    ($id: expr,
    $exchange_ids: expr,
    $include_exchange_logo: expr,
    $page: expr) => {
        crate::rust_gecko::coins::tickers(
            $id,
            $exchange_ids,
            $include_exchange_logo,
            $page,
            None,
            None,
        )
    };
    ($id: expr,
    $exchange_ids: expr,
    $include_exchange_logo: expr,
    $page: expr,
    $order: expr) => {
        crate::rust_gecko::coins::tickers
            * (
                $id,
                $exchange_ids,
                $include_exchange_logo,
                $page,
                $order,
                None,
            )
    };
    ($id: expr,
    $exchange_ids: expr,
    $include_exchange_logo: expr,
    $page: expr,
    $order: expr,
    $depth: expr) => {
        crate::rust_gecko::coins::tickers(
            $id,
            $exchange_ids,
            $include_exchange_logo,
            $page,
            $order,
            $depth,
        )
    };
}
