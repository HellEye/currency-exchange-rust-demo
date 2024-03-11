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
    Ok(res.get_display_amount(from, to, amount))
}
