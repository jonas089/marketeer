use std::time::Duration;

use balances::get_non_zero_balances;
use colored::*;
use marketeer::api::nonkyc::{
    types::{Balances, MarketWrapper},
    NonKycClient,
};
use tokio::time::sleep;
mod balances;
#[tokio::main]
async fn main() {
    let client = NonKycClient;
    loop {
        let balances: Balances = get_non_zero_balances().await;
        for balance in balances.balances {
            let usdt_pair = format!("{}{}", balance.asset, "_USDT");
            let total_balance = balance.total_balance();
            let maybe_market_data: Option<MarketWrapper> = client
                .get_token_pair(
                    marketeer::api::nonkyc::MarketDataType::MarketBySybmol,
                    &usdt_pair,
                )
                .await;

            match maybe_market_data {
                Some(market_data) => {
                    #[allow(unused)]
                    let mut asset_price: f64 = 0f64;
                    let usdt_value = match market_data {
                        MarketWrapper::MarketData(market_data) => {
                            asset_price = market_data.last_price.parse::<f64>().unwrap();
                            asset_price * total_balance
                        }
                        MarketWrapper::TickerData(ticker_data) => {
                            asset_price = ticker_data.last_price.parse::<f64>().unwrap();
                            asset_price * total_balance
                        }
                    };
                    // only show hodlings worth at least 10$
                    // hodling is not a spelling mistake!
                    if usdt_value > 10f64 {
                        println!(
                            "[{}]: \n Amount: {}, Price: {}$, Value: {}$",
                            balance.asset.red().bold(),
                            total_balance,
                            asset_price.to_string().yellow(),
                            usdt_value.to_string().green()
                        );
                    }
                }
                None => {}
            }
        }
        sleep(Duration::from_millis(3000)).await;
    }
}
