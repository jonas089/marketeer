use crate::api::constants::{BASE, MARKET_BY_SYMBOL, TICKER_BY_SYMBOL};
use crate::types::nonkyc::{MarketData, MarketWrapper, TickerData};
use reqwest::{get, Error};
pub enum BySymbolEnum {
    MarketBySybmol,
    TickerBySymbol,
}

pub struct NonKycClient;

impl NonKycClient {
    pub async fn get_by_symbol(
        &self,
        kind: BySymbolEnum,
        symbol: &str,
    ) -> Result<MarketWrapper, Error> {
        let url = match kind {
            BySymbolEnum::TickerBySymbol => {
                format!("{}{}{}", BASE, TICKER_BY_SYMBOL, symbol)
            }
            BySymbolEnum::MarketBySybmol => {
                format!("{}{}{}", BASE, MARKET_BY_SYMBOL, symbol)
            }
        };
        let response = get(url).await;
        match response {
            Ok(response) => {
                let response_json = response.text().await.unwrap();
                match kind {
                    BySymbolEnum::TickerBySymbol => {
                        let ticker_object: TickerData =
                            serde_json::from_str(&response_json).unwrap();
                        return Ok(MarketWrapper::TickerData(ticker_object));
                    }
                    BySymbolEnum::MarketBySybmol => {
                        let market_object: MarketData =
                            serde_json::from_str(&response_json).unwrap();
                        return Ok(MarketWrapper::MarketData(market_object));
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{BySymbolEnum, NonKycClient};
    #[tokio::test]
    async fn test_get_by_symbol() {
        let symbol = "BTC_USDT";
        let client: NonKycClient = NonKycClient;
        let maybe_ticker_data = client
            .get_by_symbol(BySymbolEnum::TickerBySymbol, symbol)
            .await;
        let maybe_market_data = client
            .get_by_symbol(BySymbolEnum::MarketBySybmol, symbol)
            .await;

        let ticker_data = maybe_ticker_data.unwrap();
        let market_data = maybe_market_data.unwrap();

        println!(
            "Ticker data: {:?} \n Market data: {:?}",
            &ticker_data, &market_data
        );
    }
}
