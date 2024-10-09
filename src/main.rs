mod api;
mod ntfy;
mod types;
use std::time::Duration;

use api::nonkyc::NonKycClient;
use ntfy::NftyClient;
use types::nonkyc::{MarketData, MarketWrapper, TickerData};

// Luckycoin loop
async fn nonkyc(pairs: &[&str]) {
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
                        }
                        MarketWrapper::TickerData(ticker_data) => {
                            message =
                                format!("{}: {}", &pair, ticker_data.simple_price().to_string());
                        }
                    }
                    let _ = ntfy_client.submit(message, &pair).await;
                }
                Err(_) => {}
            };
        }
        tokio::time::sleep(Duration::from_secs(600)).await;
    }
}

#[tokio::main]
async fn main() {
    let pairs = ["LKY_USDT", "BEL_USDT", "TDC_USDT"];
    nonkyc(&pairs).await;
}
