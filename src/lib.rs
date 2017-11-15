//! # binance_rust_cli
//!
//! 'binance_rust_cli' is a small command line tool for calling the Binance Crytocurrency
//!  Exchange's public API.
//!
//! Current functionality is limited to simply calling a symbol's current price and printing
//! accordingly to the shell.


#[derive(Debug)]
pub struct Binance {
    pub symbol: String,
}

impl Binance {
    pub fn new(mut args: std::env::Args) -> Result<Binance, &'static str> {
        args.next();

        let symbol = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a symbol to return"),
        };

        Ok(Binance{symbol})
    }
}