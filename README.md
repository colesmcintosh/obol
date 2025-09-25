# Obol

Fast, friendly Rust CLI for live crypto prices.

## Why this is a good demo

- Idiomatic CLI parsing with `clap`
- Async networking with `reqwest` and `tokio`
- Typed JSON handling with `serde`
- Clean error handling with `thiserror`
- Polished terminal output via `tabled` and `colored`
- Ready for CI and publishing

## Install

```bash
git clone https://github.com/colesmcintosh/obol
cd obol
cargo install --path .
```

Or run without installing:

```bash
cargo run -- BTC ETH SOL
```

## Usage

```bash
# Single coin
obol BTC

# Multiple coins
obol BTC ETH SOL

# JSON output
obol BTC --json

# History flag is scaffolded for future expansion
obol ETH --history 7d
```

### Example

```
┌─────────────┬────────┬───────────┬────────────┐
│ Coin        │ Symbol │ Price USD │ 24h Change │
├─────────────┼────────┼───────────┼────────────┤
│ Bitcoin     │ BTC    │ $64201.11 │ +2.10 %    │
│ Ethereum    │ ETH    │ $3111.42  │ -0.40 %    │
└─────────────┴────────┴───────────┴────────────┘
```

## Notes

* Data source is CoinGecko public API. Be mindful of rate limits.
* History flag is reserved for a follow up that calls the market chart endpoint.

## Roadmap

* History charts with ASCII sparklines
* Local cache for symbols and IDs
* Top N movers
* Output to CSV
* Config file in `$XDG_CONFIG_HOME/obol/config.toml`
