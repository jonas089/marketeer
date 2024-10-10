mod api;
mod ntfy;
mod types;
use std::{collections::HashMap, time::Duration};

use api::nonkyc::NonKycClient;
use ntfy::NftyClient;
use std::env;
use types::nonkyc::{MarketData, MarketWrapper, TickerData};
// Luckycoin loop
async fn nonkyc(pairs: &[&str], balances: HashMap<&str, f64>, sleep_duration: u64) {
    loop {
        let nonkyc_client = NonKycClient;
        let ntfy_client = NftyClient {
            client: reqwest::Client::new(),
        };
        for pair in pairs {
            let data = nonkyc_client
                .get_by_symbol(api::nonkyc::BySymbolEnum::TICKER_BY_SYMBOL, &pair)
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
async fn main() {
    let mut balances: HashMap<&str, f64> = HashMap::new();
    let pairs = [
        "LKY_USDT",
        "BEL_USDT",
        "TDC_USDT",
        "HTN_USDT",
        "HOLE_USDT",
        "FREN_USDT",
        "SLOPY_USDT",
        "NYANTE_USDT",
        "PHIL_USDT",
        "SMCN_USDT",
        "WOW_USDT",
    ];
    balances.insert("LKY_USDT", 302.0);
    balances.insert("BEL_USDT", 1300.0);
    balances.insert("TDC_USDT", 1250.0);
    balances.insert("HTN_USDT", 105272.0);
    balances.insert("HOLE_USDT", 3985538229171.0);
    balances.insert("FREN_USDT", 7923281.0);
    balances.insert("SLOPY_USDT", 67482057975.0);
    balances.insert("NYANTE_USDT", 403502933732.0);
    balances.insert("PHIL_USDT", 14228012387.0);
    balances.insert("SMCN_USDT", 12463.0);
    balances.insert("WOW_USDT", 38.0);
    let sleep_duration: u64 = env::var("SLEEP_DURATION")
        .unwrap_or("300".to_string())
        .parse()
        .unwrap();
    nonkyc(&pairs, balances, sleep_duration).await;
}
