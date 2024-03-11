pub enum ApiError {
    RequestError(reqwest_middleware::Error),
    JsonError(serde_json::Error),
    // Unused
    _RequestErrorStatusCode(reqwest::StatusCode),
}
// For easy error propagation, implementing From trait for common request/parsing errors
impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::JsonError(err)
    }
}

impl From<reqwest_middleware::Error> for ApiError {
    fn from(err: reqwest_middleware::Error) -> Self {
        ApiError::RequestError(err)
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::RequestError(err.into())
    }
}
