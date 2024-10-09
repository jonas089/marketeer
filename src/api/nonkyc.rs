use crate::types::nonkyc::{MarketData, MarketWrapper, TickerData};
use reqwest::{get, Error};
const BASE: &'static str = "https://api.nonkyc.io/api/v2";
const MARKET_BY_SYMBOL: &'static str = "/market/getbysymbol/";
const TICKER_BY_SYMBOL: &'static str = "/ticker/";

pub enum BySymbolEnum {
    MARKET_BY_SYMBOL,
    TICKER_BY_SYMBOL,
}

pub struct NonKycClient;

impl NonKycClient {
    pub async fn get_by_symbol(
        &self,
        kind: BySymbolEnum,
        symbol: &str,
    ) -> Result<MarketWrapper, Error> {
        let url = match kind {
            BySymbolEnum::TICKER_BY_SYMBOL => {
                format!("{}{}{}", BASE, TICKER_BY_SYMBOL, symbol)
            }
            BySymbolEnum::MARKET_BY_SYMBOL => {
                format!("{}{}{}", BASE, MARKET_BY_SYMBOL, symbol)
            }
        };
        let response = get(url).await;
        match response {
            Ok(response) => {
                let response_json = response.text().await.unwrap();
                match kind {
                    BySymbolEnum::TICKER_BY_SYMBOL => {
                        let ticker_object: TickerData =
                            serde_json::from_str(&response_json).unwrap();
                        return Ok(MarketWrapper::TickerData(ticker_object));
                    }
                    BySymbolEnum::MARKET_BY_SYMBOL => {
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
            .get_by_symbol(BySymbolEnum::TICKER_BY_SYMBOL, symbol)
            .await;
        let maybe_market_data = client
            .get_by_symbol(BySymbolEnum::MARKET_BY_SYMBOL, symbol)
            .await;

        maybe_ticker_data.unwrap();
        maybe_market_data.unwrap();
    }
}
