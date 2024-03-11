use clap::{Parser, Subcommand};
#[derive(Debug, Subcommand)]
pub enum Action {
    Convert {
        from: String,
        to: String,
        amount: f32,
    },
    List {
        base: Option<String>,
    },
    Check {
        from: String,
        to: String,
    },
    Quota,
}
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Action,
}
