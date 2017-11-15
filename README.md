# binance__rust__cli

This is a simple command line tool for accessing the Binance Cryptocurrency Exchange's public API.

# Installation

Clone the repository and build the binary with `cargo build`.

# Use

To use from bash:

`binance_rust_cli ethbtc`   

This returns and prints to bash the symbol and current pair listed. It is case insenstive and accepts a single symbol per call.

# Todo

- Bring in additional endpoints from the Binance public API (possibly private API)
- Re-structure for better Rust based documentation
- Include `clap` and more comprehensive command line functionality
- Better and more descriptive Error control
- Testing

# License

MIT