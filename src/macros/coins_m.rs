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
