//! Quote command implementations.

use clap::Args;
use uuid::Uuid;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct QuotesCommands {
    #[command(subcommand)]
    command: QuotesSubcommand,
}

#[derive(clap::Subcommand)]
enum QuotesSubcommand {
    /// Get a quote by ID
    Get {
        /// Profile ID
        profile_id: i64,
        /// Quote ID (UUID)
        quote_id: String,
    },
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: QuotesCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        QuotesSubcommand::Get {
            profile_id,
            quote_id,
        } => {
            let uuid: Uuid = quote_id.parse()?;
            let quote = client.quotes().get(profile_id, uuid).await?;
            output::print(&quote, format);
        }
    }
    Ok(())
}
