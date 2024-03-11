use crate::{
    response::CurrencyPair,
    util::{client::CacheClient, error::ApiError},
};

pub async fn check(client: &CacheClient, from: String, to: String) -> Result<String, ApiError> {
    let res: CurrencyPair = client.get(&format!("pair/{}/{}", from, to)).await?;
    Ok(format!(
        "1 {} is {:.4} {}\nLast Updated: {} UTC",
        from,
        res.conversion_rate,
        to,
        res.time_last_update_unix.format("%Y-%m-%d %H:%M:%S")
    ))
}
