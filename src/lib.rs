//! # bittrex-api
//!
//! **bittrex-api** provides a wrapper for the [Bittrex API](https://bittrex.com/home/api).
//! This crate makes it easy to consume the **Bittrex API** in Rust.
//! 
//! ##Example
//! 
//! ```no_run
//! extern crate bittrex_api;
//!
//! use bittrex_api::BittrexClient;
//! # fn main() {
//! let bittrex_client = BittrexClient::new("KEY".to_string(), "SECRET".to_string()); // Initialize the Bittrex Client with your API Key and Secret
//! let markets = bittrex_client.get_markets().unwrap(); //Get all available markets of Bittrex
//! # }
//! ```
extern crate time;
extern crate hmac;
extern crate sha2;
extern crate generic_array;

extern crate reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod error;
pub mod values;

mod client;
pub use client::BittrexClient;
