use clap::{Parser, Subcommand};
#[derive(Debug, Subcommand)]
pub enum Action {
    /// Convert currencies
    Convert {
        /// Currency to convert from
        from: String,
        /// Currency to convert to
        to: String,
        /// Amount to convert (default 1)
        #[arg(default_value = "1")]
        amount: f32,
    },
    /// List all avaliable currencies
    List {
        /// List conversion rates for given currency
        base: Option<String>,
    },
    /// Check conversion rate (equivalent to convert <from> <to> 1)
    Check {
        /// Currency to convert from
        from: String,
        /// Currency to convert to
        to: String,
    },
    /// Check api quota
    Quota,
    /// Run in interactive mode
    Interactive,
}
#[derive(Parser, Debug)]
/// Convert currencies and check exchange rates
pub struct Args {
    #[command(subcommand)]
    pub command: Action,
}
