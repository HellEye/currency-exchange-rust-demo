use crate::util::time::LastUpdateTime;
use chrono::{DateTime, Utc};

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
impl CurrencyPair {
    pub fn get_display_amount(&self, from: String, to: String, amount: f32) -> String {
        format!(
            "{:.4} {} is {:.4} {}\nLast Updated: {} UTC",
            amount,
            from,
            self.conversion_rate * amount,
            to,
            self.time_last_update_unix.format("%Y-%m-%d %H:%M:%S")
        )
    }
    pub fn get_display(&self, from: String, to: String) -> String {
        self.get_display_amount(from, to, 1.0)
    }
}
