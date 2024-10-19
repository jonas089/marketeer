use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum MarketWrapper {
    MarketData(MarketData),
    TickerData(TickerData),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketData {
    #[serde(rename = "id")]
    pub id: String,
    pub symbol: String,
    #[serde(rename = "primaryName")]
    pub primary_name: String,
    #[serde(rename = "primaryTicker")]
    pub primary_ticker: String,
    #[serde(rename = "lastPrice")]
    pub last_price: String,
    #[serde(rename = "yesterdayPrice")]
    pub yesterday_price: String,
    #[serde(rename = "highPrice")]
    pub high_price: String,
    #[serde(rename = "lowPrice")]
    pub low_price: String,
    #[serde(rename = "volume")]
    pub volume: String,
    #[serde(rename = "lastTradeAt")]
    pub last_trade_at: i64,
    #[serde(rename = "priceDecimals")]
    pub price_decimals: u32,
    #[serde(rename = "quantityDecimals")]
    pub quantity_decimals: u32,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "primaryAsset")]
    pub primary_asset: String,
    #[serde(rename = "secondaryAsset")]
    pub secondary_asset: String,
    #[serde(rename = "imageUUID")]
    pub image_uuid: String,
    #[serde(rename = "engineId")]
    pub engine_id: u32,
    #[serde(rename = "isPaused")]
    pub is_paused: bool,
    #[serde(rename = "bestAsk")]
    pub best_ask: String,
    #[serde(rename = "bestBid")]
    pub best_bid: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
    #[serde(rename = "primaryUsdValue")]
    pub primary_usd_value: String,
    #[serde(rename = "primaryCirculation")]
    pub primary_circulation: String,
    #[serde(rename = "secondaryUsdValue")]
    pub secondary_usd_value: String,
    #[serde(rename = "secondaryCirculation")]
    pub secondary_circulation: String,
    #[serde(rename = "lastPriceUpDown")]
    pub last_price_up_down: String,
    #[serde(rename = "spreadPercent")]
    pub spread_percent: String,
    #[serde(rename = "changePercent")]
    pub change_percent: String,
    #[serde(rename = "volumeSecondary")]
    pub volume_secondary: String,
    #[serde(rename = "lastPriceNumber")]
    pub last_price_number: f64,
    #[serde(rename = "bestBidNumber")]
    pub best_bid_number: f64,
    #[serde(rename = "bestAskNumber")]
    pub best_ask_number: f64,
    #[serde(rename = "yesterdayPriceNumber")]
    pub yesterday_price_number: f64,
    #[serde(rename = "changePercentNumber")]
    pub change_percent_number: f64,
    #[serde(rename = "highPriceNumber")]
    pub high_price_number: f64,
    #[serde(rename = "lowPriceNumber")]
    pub low_price_number: f64,
    #[serde(rename = "volumeNumber")]
    pub volume_number: f64,
    #[serde(rename = "volumeSecondaryNumber")]
    pub volume_secondary_number: f64,
    #[serde(rename = "volumeUsdNumber")]
    pub volume_usd_number: f64,
    #[serde(rename = "marketcapNumber")]
    pub market_cap_number: f64,
    #[serde(rename = "lineChart")]
    pub line_chart: String,
    #[serde(rename = "minimumQuantity")]
    pub minimum_quantity: u32,
    #[serde(rename = "maxAllowedPrice")]
    pub max_allowed_price: String,
    #[serde(rename = "minAllowedPrice")]
    pub min_allowed_price: String,
    #[serde(rename = "pauseBuys")]
    pub pause_buys: bool,
    #[serde(rename = "pauseSells")]
    pub pause_sells: bool,
    #[serde(rename = "assignedWebsites")]
    pub assigned_websites: String,
    #[serde(rename = "spreadPercentNumber")]
    pub spread_percent_number: f64,
}
impl MarketData {
    pub fn simple_price(&self) -> &str {
        &self.last_price
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Balances {
    pub balances: Vec<Balance>,
}
impl Balances {
    pub fn get_non_zero(&self) -> Balances {
        let mut result: Vec<Balance> = vec![];
        for balance in &self.balances {
            if balance.total_balance() > 0f64 {
                result.push(balance.clone());
            }
        }
        Balances { balances: result }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub asset: String,
    pub name: String,
    pub available: String,
    pub pending: String,
    pub held: String,
    pub assetid: String,
}

impl Balance {
    pub fn total_balance(&self) -> f64 {
        self.available.parse::<f64>().unwrap()
            + self.pending.parse::<f64>().unwrap()
            + self.held.parse::<f64>().unwrap()
    }
}
