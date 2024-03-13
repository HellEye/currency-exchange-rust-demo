// disable warning because tests aren't detected as "used"
#![allow(dead_code)]
use super::error::ErrorCode;
use http::{self, StatusCode};
use reqwest::Response;
use reqwest_middleware::{Middleware, Next};
use serde_json::json;
use task_local_extensions::Extensions;

#[derive(Default)]
/// Simple middleware for mocking responses
/// It'll capture all requests and return the response provided in the constructor
///
/// When constructing the middleware, pass in the mocked response
/// and it will be returned from all client calls
///
/// I tried to do this with the provided extensions,
/// but it seems to be beyond my abilities, as it needs a Request object
/// made from scratch, which defeats the purpose of testing functions that just use client.get
pub struct MockResponse {
    result: MockedResponse,
}

impl MockResponse {
    pub fn new(result: MockedResponse) -> Self {
        Self { result }
    }
    pub fn new_ok<S: serde::Serialize>(result: S) -> Self {
        Self::new(MockedResponse::Ok(
            serde_json::to_string(&result).expect("Parsing error"),
        ))
    }
    pub fn new_err(e: ErrorCode) -> Self {
        Self::new(MockedResponse::Err(e))
    }
}

#[derive(Clone, Debug)]
pub enum MockedResponse {
    Ok(String),
    Err(ErrorCode),
}

impl Default for MockedResponse {
    fn default() -> Self {
        MockedResponse::Err(ErrorCode::UnknownError(
            "Proper mocked response not provided".into(),
        ))
    }
}

fn make_response(status_code: http::StatusCode, body: String) -> Response {
    let response: http::response::Response<String> = http::response::Builder::new()
        .status(status_code)
        .body(body)
        .unwrap();
    response.into()
}

impl Middleware for MockResponse {
    fn handle<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        _req: reqwest::Request,
        _extensions: &'life1 mut Extensions,
        _next: Next<'life2>,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = reqwest_middleware::Result<reqwest::Response>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        let expected = self.result.clone();

        match expected {
            MockedResponse::Ok(s) => Box::pin(async move { Ok(make_response(StatusCode::OK, s)) }),
            MockedResponse::Err(e) => Box::pin(async move {
                Ok(make_response(
                    // Mocking just a generic error code with proper body
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({
                        "result": "error",
                        "error-type": e
                    })
                    .to_string(),
                ))
            }),
        }
    }
}
