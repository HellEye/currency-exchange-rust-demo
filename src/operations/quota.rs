use crate::{
    response::QuotaResponse,
    util::{client::CacheClient, error::ApiError},
};

pub async fn quota(client: &CacheClient) -> Result<String, ApiError> {
    let res: QuotaResponse = client.get_no_cache("quota").await?;
    Ok(format!("{} requests remaining", res.requests_remaining))
}
