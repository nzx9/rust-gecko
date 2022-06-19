/// Macro for list all supported coins price, market cap, volume, and market related data
/// alias: rust_gecko::coins::markets
#[macro_export]
macro_rules! coins_markets {
    ($vs_currency: expr) => {
        rust_gecko::coins::markets($vs_currency, None, None, None);
    };
    ($vs_currency: expr, $ids: expr) => {
        rust_gecko::coins::markets($vs_currency, $ids, None, None);
    };
    ($vs_currency: expr, $ids: expr, $category: expr) => {
        rust_gecko::coins::markets($vs_currency, $ids, $category, None);
    };
    ($vs_currency: expr, $ids: expr, $category: expr, $order: expr) => {
        rust_gecko::coins::markets($vs_currency, $ids, $category, $order);
    };
}
