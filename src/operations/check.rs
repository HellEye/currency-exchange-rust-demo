use crate::{
    response::CurrencyPair,
    util::{client::CacheClient, error::ApiError},
};

pub async fn check(client: &CacheClient, from: String, to: String) -> Result<String, ApiError> {
    let res: CurrencyPair = client.get(&format!("pair/{}/{}", from, to)).await?;
    Ok(res.get_display(from, to))
}
