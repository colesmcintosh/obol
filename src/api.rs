use crate::error::{ObolError, Result};
use crate::models::{SimplePrice, TickerRow};
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

const CG_SIMPLE: &str =
    "https://api.coingecko.com/api/v3/simple/price?vs_currencies=usd&include_24hr_change=true";
const CG_SEARCH: &str = "https://api.coingecko.com/api/v3/search";

#[derive(Clone, Debug)]
struct CoinMatch {
    id: String,
    name: String,
    rank: u64,
}

#[derive(Debug, Deserialize)]
struct SearchCoin {
    id: String,
    symbol: String,
    name: String,
    market_cap_rank: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    coins: Vec<SearchCoin>,
}

pub struct PriceResponse {
    pub rows: Vec<TickerRow>,
    pub missing: Vec<String>,
}

pub struct Api {
    client: Client,
}

impl Api {
    pub async fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent(format!("obol/{}", env!("CARGO_PKG_VERSION")))
            .build()?;
        Ok(Self { client })
    }

    async fn resolve_symbol(&self, symbol: &str) -> Result<Option<CoinMatch>> {
        let query = symbol.trim();
        if query.is_empty() {
            return Ok(None);
        }

        let response = self
            .client
            .get(CG_SEARCH)
            .query(&[("query", query)])
            .send()
            .await?
            .json::<SearchResponse>()
            .await?;

        let key = query.to_uppercase();
        let mut best: Option<CoinMatch> = None;

        for coin in response.coins.into_iter() {
            if !coin.symbol.eq_ignore_ascii_case(&key) {
                continue;
            }
            let rank = coin.market_cap_rank.unwrap_or(u64::MAX);
            let candidate = CoinMatch {
                id: coin.id,
                name: coin.name,
                rank,
            };
            match &best {
                Some(current) if current.rank <= candidate.rank => {}
                _ => best = Some(candidate),
            }
        }

        Ok(best)
    }

    pub async fn prices(&self, symbols: &[String]) -> Result<PriceResponse> {
        if symbols.is_empty() {
            return Err(ObolError::NoSymbols);
        }

        let mut ids = Vec::new();
        let mut id_to_symbol = HashMap::new();
        let mut name_map = HashMap::new();
        let mut missing = Vec::new();
        let mut cache: HashMap<String, CoinMatch> = HashMap::new();

        for symbol in symbols {
            let key = symbol.to_uppercase();
            let entry = if let Some(existing) = cache.get(&key) {
                existing.clone()
            } else {
                match self.resolve_symbol(symbol).await? {
                    Some(found) => {
                        cache.insert(key.clone(), found.clone());
                        found
                    }
                    None => {
                        missing.push(key);
                        continue;
                    }
                }
            };

            ids.push(entry.id.clone());
            id_to_symbol.insert(entry.id.clone(), key.clone());
            name_map.insert(entry.id.clone(), entry.name);
        }

        if ids.is_empty() {
            return Err(ObolError::UnknownSymbols(missing.join(", ")));
        }

        let url = format!("{CG_SIMPLE}&ids={}", ids.join(","));
        let raw = self.client.get(&url).send().await?.json::<Value>().await?;

        let mut rows = Vec::new();
        for id in ids {
            let Some(obj) = raw.get(&id) else {
                return Err(ObolError::BadApiShape);
            };
            let price: SimplePrice =
                serde_json::from_value(obj.clone()).map_err(|_| ObolError::BadApiShape)?;
            let symbol = id_to_symbol.get(&id).cloned().unwrap_or_default();
            let name = name_map.get(&id).cloned().unwrap_or_else(|| symbol.clone());
            rows.push(TickerRow {
                symbol,
                name,
                price_usd: price.usd,
                change_24h: price.usd_24h_change,
            });
        }

        if rows.is_empty() {
            Err(ObolError::BadApiShape)
        } else {
            Ok(PriceResponse { rows, missing })
        }
    }
}
