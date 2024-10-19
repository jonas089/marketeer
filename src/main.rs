pub mod api;
pub mod ntfy;
pub mod types;
use std::{collections::HashMap, time::Duration};

use api::nonkyc::NonKycClient;
use types::nonkyc::MarketWrapper;
// Luckycoin loop
async fn nonkyc(pairs: &[&str], balances: HashMap<&str, f64>, sleep_duration: u64) {
    loop {
        let nonkyc_client = NonKycClient;
        for pair in pairs {
            let data = nonkyc_client
                .get_by_symbol(api::nonkyc::BySymbolEnum::TickerBySymbol, &pair)
                .await;
            match data {
                Ok(d) => {
                    let mut message = "".to_string();
                    match d {
                        MarketWrapper::MarketData(market_data) => {
                            message =
                                format!("{}: {}", &pair, market_data.simple_price().to_string());
                            let price = market_data
                                .simple_price()
                                .to_string()
                                .parse::<f64>()
                                .unwrap();

                            println!("{}, {}$", message, price * balances.get(pair).unwrap());
                        }
                        MarketWrapper::TickerData(ticker_data) => {
                            message =
                                format!("{}: {}", &pair, ticker_data.simple_price().to_string());
                            let price = ticker_data
                                .simple_price()
                                .to_string()
                                .parse::<f64>()
                                .unwrap();
                            println!("{}, {}$", message, price * balances.get(pair).unwrap());
                        }
                    }
                    // let _ = ntfy_client.submit(message, &pair).await;
                }
                Err(_) => {}
            };
        }
        tokio::time::sleep(Duration::from_secs(sleep_duration)).await;
    }
}

#[tokio::main]
async fn main() {}
