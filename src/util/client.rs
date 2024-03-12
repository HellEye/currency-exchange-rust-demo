use std::time::Duration;

use dotenv_codegen::dotenv;
use http_cache_reqwest::{
    CACacheManager, Cache, CacheMode, CacheOptions, HttpCache, HttpCacheOptions,
};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

use super::error::{response_into_error, ApiError};
/// Wrapper for reqwest::Client with built in caching
pub struct CacheClient {
    pub client: ClientWithMiddleware,
    key: String,
}

impl Default for CacheClient {
    /// Create new CacheClient with default caching behavior
    fn default() -> Self {
        let key = dotenv!("EXCHANGE_API_KEY");
        let client = ClientBuilder::new(Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager::default(),
                options: HttpCacheOptions {
                    cache_options: Some(CacheOptions {
                        immutable_min_time_to_live: Duration::from_secs(
                            60 * dotenv!("CACHE_TIME_MINUTES").parse().unwrap_or(10),
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            }))
            .build();
        CacheClient {
            client,
            key: key.to_string(),
        }
    }
}

impl CacheClient {
    /// Call exchange rate api for given url
    pub async fn get<T: for<'de> serde::Deserialize<'de>>(&self, url: &str) -> Result<T, ApiError> {
        // Make request
        let res = self.client.get(self.exchange_url(url)).send().await?;
        let result = res.error_for_status_ref();
        if result.is_err() {
            let parsed = response_into_error(res).await;
            return Err(parsed);
        }

        // Parse response into desired result
        let data = serde_json::from_str(res.text().await?.as_str())?;
        Ok(data)
    }
    pub async fn get_no_cache<T: for<'de> serde::Deserialize<'de>>(
        &self,
        url: &str,
    ) -> Result<T, ApiError> {
        let res = reqwest::get(self.exchange_url(url)).await?;

        let data = serde_json::from_str(
            res.text()
                .await
                .expect("Something went wrong when reading response")
                .as_str(),
        )?;
        Ok(data)
    }

    /// Get response from exchange rate api
    ///
    /// https://exchangerate-api.com/docs
    ///
    /// Requires EXCHANGE_API_KEY in .env file
    fn exchange_url(&self, url: &str) -> String {
        format!("https://v6.exchangerate-api.com/v6/{}/{}", self.key, url)
    }
}
