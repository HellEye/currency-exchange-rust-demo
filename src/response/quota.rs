use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct QuotaResponse {
    pub requests_remaining: i32,
}
