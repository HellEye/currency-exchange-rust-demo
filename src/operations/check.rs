use crate::{
    response::CurrencyPair,
    util::{
        client::CacheClient,
        error::{map_response_error, ApiError},
    },
};

pub async fn check(client: &CacheClient, from: String, to: String) -> Result<String, ApiError> {
    let res: CurrencyPair =
        client
            .get(&format!("pair/{}/{}", from, to))
            .await
            .map_err(map_response_error(format_args!(
                "{} or {} is not a supported currency code",
                from, to
            )))?;
    Ok(res.get_display(from, to))
}
