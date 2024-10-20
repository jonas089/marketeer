pub mod constants;
pub mod types;
use std::collections::HashMap;

use constants::BASE;
use reqwest::get;
use types::Coins;

type Currency = String;
type Price = f64;

pub struct CoinGeckoClient;

impl CoinGeckoClient {
    pub async fn get_token_ids(&self) -> Option<Coins> {
        let url = format!("{}{}", BASE, "coins/list");
        let response = get(url).await;
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    return Some(Coins {
                        coins: serde_json::from_str(&response.text().await.unwrap()).unwrap(),
                    });
                }
                return None;
            }
            Err(_) => return None,
        }
    }
    pub async fn get_token_prices(
        &self,
        ids: &str,
        vs_currencies: &str,
    ) -> HashMap<String, HashMap<Currency, Price>> {
        let url = format!(
            "{}simple/price?ids={}&vs_currencies={}",
            BASE, ids, vs_currencies
        );
        let response = get(url).await.unwrap();
        let parsed: HashMap<String, HashMap<Currency, Price>> =
            serde_json::from_str(&response.text().await.unwrap()).unwrap();
        parsed
    }
}

#[cfg(test)]
mod tests {
    use super::CoinGeckoClient;
    #[tokio::test]
    async fn test_get_coin_list() {
        let client = CoinGeckoClient;
        let coins = client.get_token_ids().await.unwrap();
        println!("Coins: {:?}", &coins);
    }

    #[tokio::test]
    async fn test_get_coin_price() {
        let client = CoinGeckoClient;
        let ids = "bitcoin,litecoin";
        let vs_currencies = "usd,chf,eur";
        let prices = client.get_token_prices(ids, vs_currencies).await;
        println!("Prices: {:?}", &prices);
    }
}
