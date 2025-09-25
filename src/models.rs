use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SimplePrice {
    #[serde(default)]
    pub usd: f64,
    #[serde(default)]
    pub usd_24h_change: f64,
}

#[derive(Debug)]
pub struct TickerRow {
    pub symbol: String,
    pub name: String,
    pub price_usd: f64,
    pub change_24h: f64,
}
