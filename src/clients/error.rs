use super::JsonRpcResponse;

#[derive(Debug)]
pub enum Error {
    // Error when send http request failed
    NetworkError(reqwest::Error),
    // Response http status is not 200
    InvalidHTTPStatus(String, reqwest::StatusCode),
    // Response body can't be decoded as json-rpc response
    InvalidHTTPResponse(reqwest::Error),
    // Decode response result to specific data type failed
    DeserializeResponseJsonError(serde_json::Error),
    // There was a timeout waiting for the response
    ResponseTimeout(String),
    // JSON-RPC Response result is null
    ResultNotFound(JsonRpcResponse),
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::NetworkError(e) => Some(e),
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

impl std::error::Error for UnexpectedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
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
    // Transaction not found, latest known block (ledger info) timestamp is more recent
    // than expiration_time_secs argument.
    // Value is the latest known block (ledger info) timestamp.
    TransactionExpired(u64),
}

impl std::fmt::Display for WaitForTransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl std::error::Error for WaitForTransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WaitForTransactionError::GetTransactionError(e) => Some(e),
            _ => None,
        }
    }
}
