extern crate dotenv_codegen;

mod command;
mod interactive;
mod operations;
mod response;
mod util;

use crate::operations::{check, convert, list, quota};
use clap::Parser;
use command::{Action, Args};
use util::client::CacheClient;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();
    let client = CacheClient::default();
    let result = match args.command {
        Action::Check { from, to } => check(&client, from, to).await,
        Action::Convert { from, to, amount } => convert(&client, from, to, amount).await,
        Action::List { base } => list(&client, base).await,
        Action::Quota => quota(&client).await,
        Action::Interactive => interactive::shell::shell(&client).await,
    };
    match result {
        Ok(result) => println!("{}", result),
        Err(err) => eprintln!("Error: {}", err.user_readable().await),
    }
}
