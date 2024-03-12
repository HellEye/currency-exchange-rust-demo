use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApiResponseError {
    pub result: String,
    #[serde(rename(deserialize = "error-type"))]
    pub error_type: String,
}

// Couldn't implement this with from/into because response.text returns a future
// I honestly don't yet know how to escape async
pub async fn response_into_error(res: reqwest::Response) -> ApiError {
    let res = res.text().await.unwrap();
    let err: ApiResponseError = match serde_json::from_str(&res) {
        Ok(err) => err,
        Err(e) => {
            return ApiError::JsonError(e);
        }
    };
    ApiError::ResponseError(err)
}

pub enum ApiError {
    RequestError(reqwest_middleware::Error),
    ResponseError(ApiResponseError),
    JsonError(serde_json::Error),
    UserFriendlyError(String),
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

impl ApiError {
    pub async fn user_readable(self) -> String {
        match self {
            ApiError::ResponseError(err) => err.error_type.to_string(),
            ApiError::RequestError(err) => format!("Request error: {}", err),
            ApiError::JsonError(err) => format!("Json error: {}", err),
            ApiError::_RequestErrorStatusCode(status_code) => {
                format!("Request error: {}", status_code)
            }
            ApiError::UserFriendlyError(err) => err,
        }
    }
}

pub enum ErrorCode {
    InvalidKey,
    InactiveAccount,
    QuotaReached,
    UnsupportedCode,
    MalformedRequest,
    UnknownError(String),
}

impl From<String> for ErrorCode {
    fn from(err: String) -> Self {
        match err.as_str() {
            "invalid-key" => ErrorCode::InvalidKey,
            "inactive-account" => ErrorCode::InactiveAccount,
            "quota-reached" => ErrorCode::QuotaReached,
            "unsupported-code" => ErrorCode::UnsupportedCode,
            "malformed-request" => ErrorCode::MalformedRequest,
            e => ErrorCode::UnknownError(e.to_owned()),
        }
    }
}
/// Formats the unsupported code error from the server using the provided format
///
/// Usage
///
/// ```
/// get(path)
/// .await
/// .map_err(
///   response_error_formatter(
///     format_args!("{} is not a valid currency code", from)
/// ))
/// ```
pub fn map_response_error(format: std::fmt::Arguments<'_>) -> impl Fn(ApiError) -> ApiError + '_ {
    move |err| -> ApiError {
        match err {
            ApiError::ResponseError(err) => {
                let code: ErrorCode = err.error_type.into();
                // Converting server response errors into nicer formats
                ApiError::UserFriendlyError(match code {
                    ErrorCode::UnknownError(e) => format!("An unknown error occured: {}", e),
                    ErrorCode::InactiveAccount => {
                        "Your account on ExchangeApi is not active".to_owned()
                    }
                    ErrorCode::QuotaReached => {
                        "Your account on ExchangeApi is out of requests".to_owned()
                    }
                    ErrorCode::InvalidKey => "Invalid API key".to_owned(),
                    ErrorCode::MalformedRequest => {
                        "Oops, the request was malformed, this shouldn't happen".to_owned()
                    }
                    ErrorCode::UnsupportedCode => format!("Error: {}", format),
                })
            }
            e => e,
        }
    }
}
