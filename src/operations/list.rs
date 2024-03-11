use crate::{
    response::{CurrencyDetails, ListResponse},
    util::{client::CacheClient, error::ApiError},
};

pub async fn list(client: &CacheClient, base: Option<String>) -> Result<String, ApiError> {
    match base {
        // Provide just the list of avaliable currencies
        None => {
            let res: ListResponse = client.get("codes").await?;
            Ok(res.get_display())
        }
        // Provide the list and every conversion avaliable for given currency
        Some(base) => {
            let res: CurrencyDetails = client.get(format!("latest/{}", base).as_str()).await?;
            Ok(res.get_display(base))
        }
    }
}
