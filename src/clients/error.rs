/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::error::Error as StdError;

use crate::SymbolError;

use super::SymbolResponse;

#[derive(Debug)]
pub enum Error {
    // Error when send http request failed
    NetworkError(reqwest::Error),
    // Error when Symbol node Response http status is 409 or 404
    SymbolError(SymbolError),
    // Response http status is not 200
    InvalidHTTPStatus(String, reqwest::StatusCode),
    // Response body can't be decoded as json-rpc response
    InvalidHTTPResponse(reqwest::Error),
    // Decode response result to specific data type failed
    DeserializeResponseJsonError(serde_json::Error),
    // There was a timeout waiting for the response
    ResponseTimeout(String),
    // JSON-RPC Response result is null
    ResultNotFound(SymbolResponse),
    // Unexpected error, should never happen, likely is a bug if it happens.
    UnexpectedError(UnexpectedError),
}

impl Error {
    pub fn unexpected_no_response(req: serde_json::Value) -> Self {
        Error::UnexpectedError(UnexpectedError::NoResponse(req))
    }
    pub fn unexpected_uncategorized(err: String) -> Self {
        Error::UnexpectedError(UnexpectedError::Uncategorized(err))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        use Error::*;
        match self.to_owned() {
            NetworkError(e) => write!(f, "{}", e),
            SymbolError(e) => write!(f, "{}", e),
            InvalidHTTPStatus(e, s) => write!(f, "{}, {}", e, s),
            InvalidHTTPResponse(e) => write!(f, "{}", e),
            DeserializeResponseJsonError(e) => write!(f, "{}", e),
            ResponseTimeout(e) => write!(f, "{}", e),
            ResultNotFound(e) => write!(f, "{:?}", e),
            UnexpectedError(e) => write!(f, "{}", e),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::NetworkError(e) => Some(e),
            Error::SymbolError(e) => Some(e),
            Error::InvalidHTTPResponse(e) => Some(e),
            Error::DeserializeResponseJsonError(e) => Some(e),
            Error::UnexpectedError(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum UnexpectedError {
    NoResponse(serde_json::Value),
    Uncategorized(String),
}

impl std::fmt::Display for UnexpectedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl StdError for UnexpectedError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum WaitForTransactionError {
    // Get account transaction error
    GetTransactionError(Error),
    // Wait timeout, value is waited duration.
    Timeout(std::time::Duration),
}

impl std::fmt::Display for WaitForTransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl StdError for WaitForTransactionError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            WaitForTransactionError::GetTransactionError(e) => Some(e),
            _ => None,
        }
    }
}
