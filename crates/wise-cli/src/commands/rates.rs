//! Exchange rate command implementations.

use clap::Args;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct RatesCommands {
    #[command(subcommand)]
    command: RatesSubcommand,
}

#[derive(clap::Subcommand)]
enum RatesSubcommand {
    /// Get exchange rate between two currencies
    Get {
        /// Source currency (e.g., GBP)
        source: String,
        /// Target currency (e.g., USD)
        target: String,
    },
    /// List all available rates
    List,
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: RatesCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        RatesSubcommand::Get { source, target } => {
            let rate = client.rates().get(&source, &target).await?;
            output::print(&rate, format);
        }
        RatesSubcommand::List => {
            let rates = client.rates().list().await?;
            output::print(&rates, format);
        }
    }
    Ok(())
}
