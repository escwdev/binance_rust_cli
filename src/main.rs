extern crate reqwest;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io::Read;

#[derive(Debug, Deserialize)]
struct Ticker {
    symbol: String,
    price: String,
}

const TICKER_URL: &'static str = "https://www.binance.com/api/v1/ticker/allPrices";

fn main() {
    // let client = reqwest::Client::new();

    let mut response = reqwest::get(TICKER_URL)
        // .send()
        .expect("Failed to send request");
    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("Failed to read response");
    let array: Vec<Ticker> = serde_json::from_str(&buf).unwrap();

    for elem in array.iter() {
        println!("{:?}", elem);
    }
}