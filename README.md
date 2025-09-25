# Obol

Obol is named after the small silver coins placed with the dead to pay Charon’s toll—here it buys you live crypto prices right in your terminal.

Fast, friendly Rust CLI for live crypto prices.

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
