use crate::{
    response::{CurrencyDetails, ListResponse},
    util::{
        client::CacheClient,
        error::{map_response_error, ApiError},
    },
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
            let res: CurrencyDetails = client
                .get(format!("latest/{}", base).as_str())
                .await
                .map_err(map_response_error(format_args!(
                    "{} is not a valid currency code",
                    base
                )))?;
            Ok(res.get_display(base))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::util::{client::CacheClient, mock_middleware::MockResponse};

    #[tokio::test]
    async fn test_list_all() {
        let client = CacheClient::with(MockResponse::new_ok(ListResponse {
            documentation: "some link".to_string(),
            result: "success".to_string(),
            terms_of_use: "some link".to_string(),
            supported_codes: [("USD", "US Dollar"), ("PLN", "Polish Zloty")]
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }));

        let result = list(&client, None).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, "USD - US Dollar\nPLN - Polish Zloty");
    }
    #[tokio::test]
    async fn test_list_single() {
        let client = CacheClient::with(MockResponse::new_ok(CurrencyDetails {
            base_code: "USD".into(),
            time_last_update_unix: chrono::DateTime::UNIX_EPOCH,
            conversion_rates: HashMap::from([("PLN".into(), 4.0), ("EUR".into(), 0.8)]),
        }));

        let result = list(&client, Some("USD".to_string())).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            r"Currency conversion for USD:
Last update at 1970-01-01 00:00:00 UTC
EUR -     0.8000
PLN -     4.0000"
        );
    }
}
