use std::fmt;
use std::error::Error as StdError;

use reqwest::Error as ReqwestError;
use serde_json;

#[derive(Debug)]
pub struct BittrexError {
    pub error_type: BittrexErrorType,
    pub message: String
}

#[derive(Debug)]
pub enum BittrexErrorType {
    APIError,
    JsonError,
    NoResults
}

impl StdError for BittrexError {
    fn description(&self) -> &str {
        match self.error_type {
            BittrexErrorType::APIError => "Error while calling Bittrex API",
            BittrexErrorType::JsonError => "Error while converting response to Json Value",
            BittrexErrorType::NoResults => "No results found"
        }
    }
}

impl fmt::Display for BittrexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error_type {
            BittrexErrorType::APIError => write!(f, "{}: {}", self.description(), self.message),
            BittrexErrorType::JsonError => write!(f, "{}: {}", self.description(), self.message),
            BittrexErrorType::NoResults => write!(f, "{} ({})!", self.description(), self.message)
        }
    }
}

impl From<serde_json::Error> for BittrexError {
    fn from(error: serde_json::Error) -> Self {
        BittrexError { error_type: BittrexErrorType::JsonError, message: error.description().to_string() }
    }
}

impl From<ReqwestError> for BittrexError {
    fn from(error: ReqwestError) -> Self {
        BittrexError { error_type: BittrexErrorType::APIError, message: error.description().to_string() }
    }
}