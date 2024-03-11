use std::collections::HashMap;

use chrono::{DateTime, Utc};
trait LastUpdateTime {
    fn get_time(&self) -> DateTime<Utc>;
    fn get_time_formatted(&self) -> String {
        self.get_time().format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }
}

// Probably don't need result, docs, terms of use
// But this is the full response
// I'll omit the unnecessary fields for other responses
// Full responses can be found on exchangerate-api docs
#[derive(serde::Deserialize)]
pub struct ListResponse {
    pub result: String,
    pub documentation: String,
    pub terms_of_use: String,
    pub supported_codes: Vec<(String, String)>,
}

#[derive(serde::Deserialize)]
pub struct CurrencyDetails {
    pub base_code: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time_last_update_unix: DateTime<Utc>,
    pub conversion_rates: HashMap<String, f32>,
}
impl LastUpdateTime for CurrencyDetails {
    fn get_time(&self) -> DateTime<Utc> {
        self.time_last_update_unix
    }
}

#[derive(serde::Deserialize)]
pub struct CurrencyPair {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time_last_update_unix: DateTime<Utc>,
    pub conversion_rate: f32,
    pub conversion_result: Option<f32>,
}
impl LastUpdateTime for CurrencyPair {
    fn get_time(&self) -> DateTime<Utc> {
        self.time_last_update_unix
    }
}

#[derive(serde::Deserialize)]
pub struct QuotaResponse {
    pub requests_remaining: i32,
}
