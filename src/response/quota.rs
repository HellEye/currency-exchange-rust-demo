#[derive(serde::Deserialize)]
pub struct QuotaResponse {
    pub requests_remaining: i32,
}
