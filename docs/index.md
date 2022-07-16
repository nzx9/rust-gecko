# rust_gecko

## Endpoint to Function Map

| Category  | End Point  | rust_gecko function  |
|:---: |:---: |:---: |
| ping  | /ping  | rust_gecko::server::ping()  |
| simple  | /simple/price  | rust_gecko::simple::price()  |
| simple  | /simple/token_price/{id}  | rust_gecko::simple::token_price()  |
| simple  | /simple/supported_vs_currencies  | rust_gecko::simple::supported_vs_currencies()  |
| coins  | /coins/list  | rust_gecko::coins::list()  |
| coins  | /coins/markets  | rust_gecko::coins::markets()  |
| coins  | /coins/{id}  | rust_gecko::coins::get()  |
| coins  | /coins/{id}/tickers  | rust_gecko::coins::tickers()  |
| coins  | /coins/{id}/history  | rust_gecko::coins::history()  |
| coins  | /coins/{id}/market_chart  | rust_gecko::coins::market_chart()  |
| coins  | /coins/{id}/market_chart/range  | rust_gecko::coins::market_chart_range()  |
| coins  | /coins/{id}/ohlc  | rust_gecko::coins::ohlc()  |
| contract  | /coins/{id}/contract/{contract_address}  | rust_gecko::contract::get()  |
| contract  | /coins/{id}/contract/{contact_address}/market_chart  | rust_gecko::contract::market_chart()  |
| contract  | /coins/{id}/contract/{contact_address}/market_chart/range  | rust_gecko::contract::market_chart_range()  |
| asset_platforms  | /asset_platforms  | rust_gecko::asset_platforms::get()  |
| categories  | /coins/categories/list  | rust_gecko::categories::list()  |
| categories  | /coins/categories  | rust_gecko::categories::get()  |
| exchanges  | /exchanges  | rust_gecko::exchanges::get()  |
| exchanges  | /exchanges/list  | rust_gecko::exchanges::list()  |
| exchanges  | /exchanges/{id}  | rust_gecko::exchanges::by_id()  |
| exchanges  | /exchanges/{id}/tickers  | rust_gecko::exchanges::tickers()  |
| exchanges  | /exchanges/{id}/volume_chart  | rust_gecko::exchanges::volume_chart()  |
| indexes  | /indexes  | rust_gecko::indexes::get()  |
| indexes  | /indexes/{market_id}/{id}  | rust_gecko::indexes::by_id()  |
| indexes  | /indexes/list  | rust_gecko::indexes::list()  |
| derivatives  | /derivatives  | rust_gecko::derivatives::get()  |
| derivatives  | /derivatives/exchanges  | rust_gecko::derivatives::exchanges()  |
| derivatives  | /derivatives/exchanges/{id}  | rust_gecko::derivatives::exchanges_id()  |
| derivatives  | /derivatives/exchanges/list  | rust_gecko::derivatives::exchanges_list()  |
| exchange_rates  | /exchange_rates  | rust_gecko::exchange_rates::get()  |
| search  | /search  | rust_gecko::search::get()  |
| trending  | /search/trending  | rust_gecko::trending::get()  |
| global  | /global  | rust_gecko::global::get()  |
| global  | /global/decentralized_finance_defi  | rust_gecko::global::defi()  |
| companies  | /companies/public_treasury/{coin_id}  | rust_gecko::companies::get()  |
