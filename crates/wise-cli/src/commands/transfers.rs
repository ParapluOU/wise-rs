//! Transfer command implementations.

use clap::Args;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct TransfersCommands {
    #[command(subcommand)]
    command: TransfersSubcommand,
}

#[derive(clap::Subcommand)]
enum TransfersSubcommand {
    /// List transfers
    List {
        /// Profile ID (optional)
        #[arg(short, long)]
        profile: Option<i64>,
        /// Maximum number of results
        #[arg(short, long)]
        limit: Option<u32>,
        /// Number of results to skip
        #[arg(long)]
        offset: Option<u32>,
    },
    /// Get a specific transfer
    Get {
        /// Transfer ID
        id: i64,
    },
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: TransfersCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        TransfersSubcommand::List {
            profile,
            limit,
            offset,
        } => {
            let transfers = client.transfers().list(profile, limit, offset).await?;
            output::print(&transfers, format);
        }
        TransfersSubcommand::Get { id } => {
            let transfer = client.transfers().get(id).await?;
            output::print(&transfer, format);
        }
    }
    Ok(())
}
