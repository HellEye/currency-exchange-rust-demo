# Currency Converter

Sample project to convert currencies using [Rust](https://www.rust-lang.org).

Uses [ExchangeRate](https://exchangerate-api.com)

## Install

1. Provide the API key in `.env` (see `.env.example`) as `EXCHANGE_API_KEY`
   1. To get the key go to [exchangerate-api.com](https://exchangerate-api.com) and create a free account.
2. Optionally, provide `CACHE_TIME_MINUTES` in `.env` (see `.env.example`) (default is 10 minutes)
3. Build the project using `cargo build --release`
4. Run the binary using `cargo run --release` or from the executable `./target/release/exchange`

## Usage

`exchange <command>`

### Commands

`exchange interactive` - Run interactive mode

`exchange convert <from> <to> <amount>` - Convert amount from one currency to another

`exchange list` - List all supported currencies

`exchange list <currency>` - List all conversion rates for a given currency

`exchange check <from> <to>` - Check conversion rate for given currencies

`exchange quota` - check remaining avaliable api calls (This request is always uncached)

## Dependencies

This app uses:

- `chrono` for date and time manipulation
- `clap` for command line arguments
- `dotenv-codegen` for environment variables
  - For portability, I've decided to use the dotenv-codegen crate, which will load .env file at build time.
  - This is a tradeoff between safety of env variables and portability. But thanks to this, the compiled executable can be stored in any location by itself.
- `reqwest` for HTTP requests
  - Initially I wanted to use the blocking version, however caching middleware requires non-blocking variant.
- `tokio` for async runtime
- `reqwest-middleware` and `http-cache-reqwest` for caching
- `serde` and `serde-json` for JSON deserialization
- `dialoguer` for interactive mode

## Extra info

- I've opted to ignore most of the return values from the api (like links to docs or terms of use, and additional metadata), they could however be added to the result structs.
- Pretty error handling isn't great because the exchange api doesn't give back any information besides a simple error code. Due to this, more context than just the response is necessary.

## TODO

- Add more tests
