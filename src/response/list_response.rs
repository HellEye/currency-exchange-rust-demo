#[derive(serde::Deserialize)]
pub struct ListResponse {
    pub result: String,
    pub documentation: String,
    pub terms_of_use: String,
    pub supported_codes: Vec<(String, String)>,
}
impl ListResponse {
    pub fn get_display(&self) -> String {
        self.supported_codes
            .iter()
            .map(|(symbol, name)| format!("{} - {}", symbol, name))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
