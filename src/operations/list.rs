use crate::{
    response::{CurrencyDetails, ListResponse},
    util::{client::CacheClient, error::ApiError},
};

pub async fn list(client: &CacheClient, base: Option<String>) -> Result<String, ApiError> {
    match base {
        // Provide just the list of avaliable currencies
        None => {
            let res: ListResponse = client.get("codes").await?;
            Ok(res
                .supported_codes
                .iter()
                .map(|(symbol, name)| format!("{} - {}", symbol, name))
                .collect::<Vec<String>>()
                .join("\n"))
        }
        // Provide the list and every conversion avaliable for given currency
        Some(base) => {
            let res: CurrencyDetails = client.get(format!("latest/{}", base).as_str()).await?;

            let data = res
                .conversion_rates
                .iter()
                .map(|(symbol, rate)| format!("{} - {:>10.4}", symbol, rate))
                .collect::<Vec<String>>()
                .join("\n");
            Ok(format!(
                "Currency conversion for {base}:\nLast update at {time}\n{data}",
                base = base,
                data = data,
                time = res.time_last_update_unix.format("%Y-%m-%d %H:%M:%S")
            ))
        }
    }
}
