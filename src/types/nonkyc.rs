use serde::{Deserialize, Serialize};

pub enum MarketWrapper {
    MarketData(MarketData),
    TickerData(TickerData),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketData {
    pub _id: String,
    pub symbol: String,
    pub primaryName: String,
    pub primaryTicker: String,
    pub lastPrice: String,
    pub yesterdayPrice: String,
    pub highPrice: String,
    pub lowPrice: String,
    pub volume: String,
    pub lastTradeAt: i64,
    pub priceDecimals: u32,
    pub quantityDecimals: u32,
    pub isActive: bool,
    pub primaryAsset: String,
    pub secondaryAsset: String,
    pub imageUUID: String,
    pub engineId: u32,
    pub isPaused: bool,
    pub bestAsk: String,
    pub bestBid: String,
    pub createdAt: i64,
    pub updatedAt: i64,
    pub primaryUsdValue: String,
    pub primaryCirculation: String,
    pub secondaryUsdValue: String,
    pub secondaryCirculation: String,
    pub lastPriceUpDown: String,
    pub spreadPercent: String,
    pub changePercent: String,
    pub volumeSecondary: String,
    pub lastPriceNumber: f64,
    pub bestBidNumber: f64,
    pub bestAskNumber: f64,
    pub yesterdayPriceNumber: f64,
    pub changePercentNumber: f64,
    pub highPriceNumber: f64,
    pub lowPriceNumber: f64,
    pub volumeNumber: f64,
    pub volumeSecondaryNumber: f64,
    pub volumeUsdNumber: f64,
    pub marketcapNumber: f64,
    pub lineChart: String,
    pub minimumQuantity: u32,
    pub maxAllowedPrice: String,
    pub minAllowedPrice: String,
    pub pauseBuys: bool,
    pub pauseSells: bool,
    pub assignedWebsites: String,
    pub spreadPercentNumber: f64,
    pub id: String,
}
impl MarketData {
    pub fn simple_price(&self) -> &str {
        &self.lastPrice
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TickerData {
    pub ticker_id: String,
    pub r#type: String, // "type" is a keyword in Rust, so prefix with `r#`
    pub base_currency: String,
    pub target_currency: String,
    pub last_price: String,
    pub base_volume: String,
    pub target_volume: String,
    pub usd_volume_est: String,
    pub bid: String,
    pub ask: String,
    pub high: String,
    pub low: String,
    pub change_percent: String,
    pub previous_day_price: String,
}
impl TickerData {
    pub fn simple_price(&self) -> &str {
        &self.last_price
    }
}
