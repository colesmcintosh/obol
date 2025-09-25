mod api;
mod cli;
mod error;
mod format;
mod models;

use crate::cli::Cli;
use crate::error::{ObolError, Result};
use clap::Parser;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let args = Cli::parse();

    if args.symbols.is_empty() {
        return Err(ObolError::NoSymbols);
    }

    let api = api::Api::new().await?;
    let api::PriceResponse { rows, missing } = api.prices(&args.symbols).await?;

    if !missing.is_empty() {
        eprintln!("warning: unknown symbol(s) skipped: {}", missing.join(", "));
    }

    if args.json {
        let payload = serde_json::json!({
            "data": rows
                .iter()
                .map(|r| serde_json::json!({
                    "symbol": r.symbol,
                    "name": r.name,
                    "price_usd": r.price_usd,
                    "change_24h": r.change_24h,
                }))
                .collect::<Vec<_>>()
        });
        let pretty = serde_json::to_string_pretty(&payload)
            .map_err(|err| ObolError::Render(err.to_string()))?;
        println!("{pretty}");
    } else {
        let table = format::to_table(&rows);
        println!("{table}");
    }

    Ok(())
}
