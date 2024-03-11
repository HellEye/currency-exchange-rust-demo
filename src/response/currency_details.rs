use crate::util::time::LastUpdateTime;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

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

impl CurrencyDetails {
    pub fn get_display(&self, base: String) -> String {
        let data = self
            .conversion_rates
            .iter()
            .map(|(symbol, rate)| format!("{} - {:>10.4}", symbol, rate))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "Currency conversion for {base}:\nLast update at {time}\n{data}",
            base = base,
            data = data,
            time = self.get_time_formatted()
        )
    }
}
