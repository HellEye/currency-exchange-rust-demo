use crate::{
    response::CurrencyPair,
    util::{client::CacheClient, error::ApiError},
};

pub async fn convert(
    client: &CacheClient,
    from: String,
    to: String,
    amount: f32,
) -> Result<String, ApiError> {
    let res: CurrencyPair = client.get(&format!("pair/{}/{}", from, to)).await?;
    Ok(format!(
        "{:.4} {} is {:.4} {}\nLast Updated: {} UTC",
        amount,
        from,
        res.conversion_rate * amount,
        to,
        res.time_last_update_unix.format("%Y-%m-%d %H:%M:%S")
    ))
}
