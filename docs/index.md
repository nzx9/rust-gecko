# rust_gecko

## Development Road Map

| Category         | End Point                                                  | Implemented  | rust_gecko function                            | rust_gecko macro                   | Stability  | Implemented Version  |
|----------------- |----------------------------------------------------------- |------------- |----------------------------------------------- |----------------------------------- |----------- |--------------------- |
| ping             | /ping                                                      | True         | rust_gecko::server::ping()                     | -                                  | Stable     | 0.0.0                |
| simple           | /simple/price                                              | True         | rust_gecko::simple::price()                    | rust_gecko::simple_price!()        | Stable     | 0.0.0                |
| simple           | /simple/token_price/{id}                                   | True         | rust_gecko::simple::token_price()              | rust_gecko::simple_token_price!()  | Stable     | 0.0.0                |
| simple           | /simple/supported_vs_currencies                            | True         | rust_gecko::simple::supported_vs_currencies()  | -                                  | Stable     | 0.0.0                |
| coins            | /coins/list                                                | True         | rust_gecko::coins::list()                      | -                                  | Stable     | 0.0.1                |
| coins            | /coins/markets                                             | True         | rust_gecko::coins::markets()                   | rust_gecko::coins_markets!()       | Stable     | 0.0.1                |
| coins            | /coins/{id}                                                | False        |                                                |                                    |            |                      |
| coins            | /coins/{id}/tickers                                        | False        |                                                |                                    |            |                      |
| coins            | /coins/{id}/history                                        | False        |                                                |                                    |            |                      |
| coins            | /coins/{id}/market_chart                                   | False        |                                                |                                    |            |                      |
| coins            | /coins/{id}/market_chart/range                             | False        |                                                |                                    |            |                      |
| coins            | /coins/{id}/ohlc                                           | False        |                                                |                                    |            |                      |
| contract         | /coins/{id}/contract/{contract_address}                    | False        |                                                |                                    |            |                      |
| contract         | /coins/{id}/contract/{contact_address}/market_chart        | False        |                                                |                                    |            |                      |
| contract         | /coins/{id}/contract/{contact_address}/market_chart/range  | False        |                                                |                                    |            |                      |
| asset_platforms  | /asset_platforms                                           | False        |                                                |                                    |            |                      |
| categories       | /coins/categories/list                                     | False        |                                                |                                    |            |                      |
| categories       | /coins/categories                                          | False        |                                                |                                    |            |                      |
| exchanges        | /exchanges                                                 | False        |                                                |                                    |            |                      |
| exchanges        | /exchanges/list                                            | False        |                                                |                                    |            |                      |
| exchanges        | /exchanges/{id}                                            | False        |                                                |                                    |            |                      |
| exchanges        | /exchanges/{id}/tickers                                    | False        |                                                |                                    |            |                      |
| exchanges        | /exchanges/{id}/volume_chart                               | False        |                                                |                                    |            |                      |
| indexes          | /indexes                                                   | False        |                                                |                                    |            |                      |
| indexes          | /indexes/{market_id}/{id}                                  | False        |                                                |                                    |            |                      |
| indexes          | /indexes/list                                              | False        |                                                |                                    |            |                      |
| derivatives      | /derivatives                                               | False        |                                                |                                    |            |                      |
| derivatives      | /derivatives/exchanges                                     | False        |                                                |                                    |            |                      |
| derivatives      | /derivatives/exchanges/{id}                                | False        |                                                |                                    |            |                      |
| derivatives      | /derivatives/exchanges/list                                | False        |                                                |                                    |            |                      |
| exchange_rates   | /exchange_rates                                            | False        |                                                |                                    |            |                      |
| search           | /search                                                    | False        |                                                |                                    |            |                      |
| trending         | /search/trending                                           | False        |                                                |                                    |            |                      |
| global           | /global                                                    | False        |                                                |                                    |            |                      |
| global           | global/decentralized_finance_defi                          | False        |                                                |                                    |            |                      |
| companies        | /companies/public_treasury/{coin_id}                       | False        |                                                |                                    |            |                      |
