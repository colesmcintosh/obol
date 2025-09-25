use crate::models::TickerRow;
use colored::*;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Row {
    #[tabled(rename = "Coin")]
    name: String,
    #[tabled(rename = "Symbol")]
    symbol: String,
    #[tabled(rename = "Price USD")]
    price: String,
    #[tabled(rename = "24h Change")]
    change: String,
}

pub fn to_table(rows: &[TickerRow]) -> String {
    if rows.is_empty() {
        return "No data returned.".to_string();
    }

    let items: Vec<Row> = rows
        .iter()
        .map(|r| {
            let change_raw = format!("{:+.2} %", r.change_24h);
            let change_str = if r.change_24h >= 0.0 {
                change_raw.green().bold().to_string()
            } else {
                change_raw.red().bold().to_string()
            };

            Row {
                name: r.name.clone(),
                symbol: r.symbol.clone(),
                price: format!("${:.2}", r.price_usd),
                change: change_str,
            }
        })
        .collect();

    Table::new(items).to_string()
}
