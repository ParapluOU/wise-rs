//! Card transaction command implementations.

use clap::Args;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct CardTransactionsCommands {
    #[command(subcommand)]
    command: CardTransactionsSubcommand,
}

#[derive(clap::Subcommand)]
enum CardTransactionsSubcommand {
    /// Get a card transaction by ID
    Get {
        /// Profile ID
        profile_id: i64,
        /// Transaction ID
        transaction_id: String,
    },
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: CardTransactionsCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        CardTransactionsSubcommand::Get {
            profile_id,
            transaction_id,
        } => {
            let tx = client
                .card_transactions()
                .get(profile_id, &transaction_id)
                .await?;
            output::print(&tx, format);
        }
    }
    Ok(())
}
