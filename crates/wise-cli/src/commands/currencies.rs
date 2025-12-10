//! Currency command implementations.

use clap::Args;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct CurrenciesCommands {
    #[command(subcommand)]
    command: CurrenciesSubcommand,
}

#[derive(clap::Subcommand)]
enum CurrenciesSubcommand {
    /// List all supported currencies
    List,
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: CurrenciesCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        CurrenciesSubcommand::List => {
            let currencies = client.currencies().list().await?;
            output::print(&currencies, format);
        }
    }
    Ok(())
}
