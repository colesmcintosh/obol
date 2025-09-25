use clap::{Parser, ValueEnum};

/// Obol: live crypto prices in your terminal
#[derive(Parser, Debug)]
#[command(
    name = "obol",
    about = "Fetch live crypto prices with optional history"
)]
pub struct Cli {
    /// One or more symbols like BTC ETH SOL
    pub symbols: Vec<String>,

    /// Show history for a lookback window like 7d or 24h
    #[arg(long)]
    pub history: Option<Lookback>,

    /// Output as JSON instead of a table
    #[arg(long)]
    pub json: bool,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Lookback {
    #[value(alias = "24h", alias = "1d")]
    H24,
    #[value(alias = "7d")]
    D7,
    #[value(alias = "30d")]
    D30,
}
