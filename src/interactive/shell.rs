use dialoguer::{FuzzySelect, Input, Select};

use crate::{
    operations::{convert, list, quota},
    util::{client::CacheClient, error::ApiError},
};
fn get_currency_code(line: String) -> String {
    line.split(' ').next().unwrap().to_string()
}
/// Make a fuzzy select and return the selected currency code
fn fuzzy_select_currency(items: &[&str], prompt: Option<&str>) -> String {
    let selection = FuzzySelect::new()
        .with_prompt(prompt.unwrap_or("Select a currency"))
        .items(items)
        .default(0)
        .interact()
        .expect("Failed to get selection");
    get_currency_code(items[selection].to_string())
}

async fn list_interactive(client: &CacheClient) -> Result<String, ApiError> {
    let choices = vec!["Rates for currency", "List currencies"];
    let selection = Select::new()
        .with_prompt("What do you want to list?")
        .items(&choices)
        .default(0)
        .interact()
        .expect("Failed to get selection");
    let currencies = list(client, None).await?;
    if selection == 1 {
        return Ok(currencies);
    }
    let currencies: Vec<&str> = currencies.split('\n').collect();
    let selected = fuzzy_select_currency(&currencies, None);
    list(client, Some(selected.to_string())).await
}

pub async fn convert_interactive(client: &CacheClient) -> Result<String, ApiError> {
    let currencies = list(client, None).await?;
    let currencies: Vec<&str> = currencies.split('\n').collect();
    let from = fuzzy_select_currency(&currencies, Some("Select the base currency"));
    let to = fuzzy_select_currency(&currencies, Some("Select the target currency"));
    let amount: f32 = Input::new()
        .default(1f32)
        .with_prompt(format!("How much {} do you want to convert?", from))
        .interact()
        .unwrap();
    convert(client, from, to, amount).await
}

pub async fn shell(client: &CacheClient) -> Result<String, ApiError> {
    let choices = vec!["Convert", "List", "Quota"];
    let selection = Select::new()
        .with_prompt("What do you want to do?")
        .items(&choices)
        .default(0)
        .interact()
        .expect("Failed to get selection");
    match selection {
        2 => quota(client).await,
        1 => list_interactive(client).await,
        0 => convert_interactive(client).await,
        _ => Ok("".to_string()),
    }
}
