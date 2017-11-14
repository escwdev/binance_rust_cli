use std::error::Error;
use std::io::prelude::*;
use std::env;

#[derive(Debug)]
pub struct Binance {
    pub query: String,
}

impl Binance {
    pub fn new(mut args: std::env::Args) -> Result<Binance, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a pair query"),
        };

        Ok(Binance{query})
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
