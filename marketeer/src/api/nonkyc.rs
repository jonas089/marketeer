use crate::api::constants::{BASE, MARKET_BY_SYMBOL, TICKER_BY_SYMBOL};
pub mod auth;
pub mod types;
use reqwest::get;
use types::{MarketData, MarketWrapper, TickerData};
pub enum MarketDataType {
    MarketBySybmol,
    TickerBySymbol,
}

pub struct NonKycClient;

impl NonKycClient {
    pub async fn get_token_pair(
        &self,
        data_type: MarketDataType,
        symbol: &str,
    ) -> Option<MarketWrapper> {
        let url = match data_type {
            MarketDataType::TickerBySymbol => {
                format!("{}{}{}", BASE, TICKER_BY_SYMBOL, symbol)
            }
            MarketDataType::MarketBySybmol => {
                format!("{}{}{}", BASE, MARKET_BY_SYMBOL, symbol)
            }
        };
        let response = get(url).await;
        match response {
            Ok(response) => {
                let response_json = response.text().await.unwrap();
                match data_type {
                    MarketDataType::TickerBySymbol => {
                        let ticker_object: Option<TickerData> =
                            serde_json::from_str(&response_json).unwrap_or(None);
                        match ticker_object {
                            Some(ticker) => return Some(MarketWrapper::TickerData(ticker)),
                            None => return None,
                        }
                    }
                    MarketDataType::MarketBySybmol => {
                        let market_object: Option<MarketData> =
                            serde_json::from_str(&response_json).unwrap_or(None);
                        match market_object {
                            Some(market) => return Some(MarketWrapper::MarketData(market)),
                            None => return None,
                        }
                    }
                }
            }
            Err(_) => return None,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{MarketDataType, NonKycClient};
    #[tokio::test]
    async fn test_get_by_symbol() {
        let symbol = "BTC_USDT";
        let client: NonKycClient = NonKycClient;
        let maybe_ticker_data = client
            .get_token_pair(MarketDataType::TickerBySymbol, symbol)
            .await;
        let maybe_market_data = client
            .get_token_pair(MarketDataType::MarketBySybmol, symbol)
            .await;

        let ticker_data = maybe_ticker_data.unwrap();
        let market_data = maybe_market_data.unwrap();

        println!(
            "Ticker data: {:?} \n Market data: {:?}",
            &ticker_data, &market_data
        );
    }
}
