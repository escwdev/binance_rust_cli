extern crate binance_rust_cli;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io::Read;
use std::env;
use std::process;

use binance_rust_cli::Config;

#[derive(Debug, Deserialize)]
struct Ticker {
    symbol: String,
    price: String,
}

const TICKER_URL: &'static str = "https://www.binance.com/api/v1/ticker/allPrices";

fn main() {
    let config = binance_rust_cli::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Probplem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut response = reqwest::get(TICKER_URL)
        .expect("Failed to send request");
    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("Failed to read response");
    let array: Vec<Ticker> = serde_json::from_str(&buf).unwrap();

    for elem in array.iter() {
        println!("{:?}", elem);
    }
}