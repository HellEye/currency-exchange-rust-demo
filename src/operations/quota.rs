use crate::{
    response::QuotaResponse,
    util::{
        client::CacheClient,
        error::{map_response_error, ApiError},
    },
};

pub async fn quota(client: &CacheClient) -> Result<String, ApiError> {
    let res: QuotaResponse = client
        .get("quota")
        .await
        .map_err(map_response_error(format_args!("")))?;
    Ok(format!("{} requests remaining", res.requests_remaining))
}
