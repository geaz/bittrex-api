use std::fmt;
use std::error::Error as StdError;

use reqwest::Error as ReqwestError;
use serde_json;

#[derive(Debug)]
pub enum BittrexError {
    APIError,
    JsonError,
    MarketNotFound
}

impl StdError for BittrexError {
    fn description(&self) -> &str {
        match *self {
            BittrexError::APIError => "Error while calling Bittrex API",
            BittrexError::JsonError => "Error while converting response to Json Value",
            BittrexError::MarketNotFound => "Market not found!"
        }
    }
}

impl fmt::Display for BittrexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BittrexError::APIError => write!(f, "Error while calling Bittrex API"),
            BittrexError::JsonError => write!(f, "Error while converting response to Json Value"),
            BittrexError::MarketNotFound => write!(f, "Market not found!")
        }
    }
}

impl From<serde_json::Error> for BittrexError {
    fn from(error: serde_json::Error) -> Self {
        BittrexError::JsonError
    }
}

impl From<ReqwestError> for BittrexError {
    fn from(error: ReqwestError) -> Self {
        BittrexError::APIError
    }
}