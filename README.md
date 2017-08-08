[![Build Status](https://travis-ci.org/geaz/bittrex-api.svg?branch=master)](https://travis-ci.org/geaz/bittrex-api)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/Geaz/bittrex-api/master/LICENSE)
[![Crates](https://img.shields.io/crates/v/bittrex-api.svg)](https://crates.io/crates/bittrex-api)

# bittrex-api

**bittrex-api** provides a wrapper for the [Bittrex API](https://bittrex.com/home/api).  
This crate makes it easy to consume the **Bittrex API** in Rust.

## Example

```
extern crate bittrex_api;

use bittrex_api::BittrexClient;

let bittrex_client = BittrexClient::new("KEY", "SECRET"); // Initialize the Bittrex Client with your API Key and Secret
let markets = bittrex_client.get_markets().unwrap(); //Get all available markets of Bittrex
```

See the [Documentation](https://docs.rs/bittrex-api/0.2.0/bittrex_api/) for more information about the various wrapper functions.