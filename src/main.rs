extern crate binance_rust_cli;
extern crate reqwest;
extern crate term;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io::Read;
use std::env;
use std::process;

#[derive(Debug, Deserialize)]
struct Ticker {
    symbol: String,
    price: String,
}

const TICKER_URL: &'static str = "https://www.binance.com/api/v1/ticker/allPrices";

fn main() {
    let binance = binance_rust_cli::Binance::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut response = reqwest::get(TICKER_URL)
        .expect("Failed to send request");
    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("Failed to read response");
    let array: Vec<Ticker> = serde_json::from_str(&buf).unwrap();

    let mut t = term::stdout().unwrap();

    for elem in array.iter() {
        if binance.symbol.to_uppercase() == elem.symbol { 
            t.attr(term::Attr::Bold).unwrap();
            t.fg(term::color::CYAN).unwrap();
            println!("Symbol: {:?}", elem.symbol);
            t.fg(term::color::YELLOW).unwrap();
            println!("Price: {:?}", elem.price); 
       };
    }
}