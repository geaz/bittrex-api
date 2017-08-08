use std::fmt;
use std::error::Error as StdError;

use reqwest::Error as ReqwestError;
use serde_json;

#[derive(Debug)]
pub struct BittrexError {
    pub error_type: BittrexErrorType,
    pub message: String,
}

#[derive(Debug)]
pub enum BittrexErrorType {
    APIError,
    JsonError,
    NoResults,
}

impl StdError for BittrexError {
    fn description(&self) -> &str {
        match self.error_type {
            BittrexErrorType::APIError => "Error while calling Bittrex API",
            BittrexErrorType::JsonError => "Error while converting response to Json Value",
            BittrexErrorType::NoResults => "No results found",
        }
    }
}

impl fmt::Display for BittrexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error_type {
            BittrexErrorType::APIError => write!(f, "{}: {}", self.description(), self.message),
            BittrexErrorType::JsonError => write!(f, "{}: {}", self.description(), self.message),
            BittrexErrorType::NoResults => write!(f, "{} ({})!", self.description(), self.message),
        }
    }
}

impl From<serde_json::Error> for BittrexError {
    fn from(error: serde_json::Error) -> Self {
        BittrexError {
            error_type: BittrexErrorType::JsonError,
            message: error.description().to_string(),
        }
    }
}

impl From<ReqwestError> for BittrexError {
    fn from(error: ReqwestError) -> Self {
        let mut err: Option<BittrexError> = None;

        if error.is_http() {
            err = match error.url() {
                Some(url) => Some(BittrexError {
                    error_type: BittrexErrorType::APIError,
                    message: format!("Problem making request to: {}", url),
                }),
                None => Some(BittrexError {
                    error_type: BittrexErrorType::APIError,
                    message: "No Url given".to_string(),
                }),
            }
        }

        if error.is_serialization() {
            err = match error.get_ref() {
                Some(err) => Some(BittrexError {
                    error_type: BittrexErrorType::APIError,
                    message: format!("Problem parsing information {}", err),
                }),
                None => Some(BittrexError {
                    error_type: BittrexErrorType::APIError,
                    message: "Problem parsing information (no info given)".to_string(),
                }),
            }
        }

        if error.is_redirect() {
            err = Some(BittrexError {
                error_type: BittrexErrorType::APIError,
                message: "Server redirecting too many times or making loop".to_string(),
            });
        }

        if err.is_none() {
            err = Some(BittrexError {
                error_type: BittrexErrorType::APIError,
                message: "Error undefined!".to_string(),
            });
        }

        err.unwrap()
    }
}
