use std::time::Duration;

use balances::get_non_zero_balances;
use colored::*;
use marketeer::api::{
    coingecko::CoinGeckoClient,
    nonkyc::{
        types::{Balances, MarketWrapper},
        NonKycClient,
    },
};
use tokio::time::sleep;
mod balances;
use prettytable::{format, row, Table};
use std::process::Command;
use textwrap::wrap;

const MINA: f64 = 5571.0;
const ZEC: f64 = 17.5;

// Wraps text but keeps the color formatting
fn print_boxed_text(content: &str, width: usize) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);

    // Wrap text without stripping ANSI, by taking into account word boundaries
    let wrapped_lines = wrap(content, width);

    // Add the wrapped lines to the table
    for line in wrapped_lines {
        table.add_row(row![line]);
    }

    table.printstd();
}

#[tokio::main]
async fn main() {
    let gecko = CoinGeckoClient;
    let client = NonKycClient;
    loop {
        let mut message: String = String::new();
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
                    if usdt_value > 10f64 {
                        message += &format!(
                            "[{}]: \n Price: {}$, Value: {}$\n",
                            balance.asset.red().bold(),
                            asset_price.to_string().yellow(),
                            usdt_value.to_string().green()
                        );
                    }
                }
                None => {}
            }
        }

        let prices = gecko.get_token_prices("mina-protocol,zcash", "usd").await;
        let mina_price = prices.get("mina-protocol").unwrap().get("usd").unwrap();
        let zec_price = prices.get("zcash").unwrap().get("usd").unwrap();
        message += &format!(
            "[{}]: \n Price: {}$, Value: {}$\n",
            "MINA".red().bold(),
            mina_price.to_string().yellow(),
            (MINA * mina_price).to_string().green()
        );
        message += &format!(
            "[{}]: \n Price: {}$, Value: {}$\n",
            "ZEC".red().bold(),
            zec_price.to_string().yellow(),
            (ZEC * zec_price).to_string().green()
        );
        Command::new("clear").status().expect("Failed to clear cmd");
        print_boxed_text(&message, 100);
        sleep(Duration::from_millis(30000)).await;
    }
}
#[tokio::test]
async fn test_get_mina_zec() {
    let gecko = CoinGeckoClient;
    let prices = gecko.get_token_prices("mina-protocol,zcash", "usd").await;
    let mina_price = prices.get("mina-protocol").unwrap().get("usd").unwrap();
    let zec_price = prices.get("zcash").unwrap().get("usd").unwrap();
    println!("Mina: {:?}, Zec: {:?}", &mina_price, zec_price);
}
