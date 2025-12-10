//! Balance command implementations.

use clap::Args;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct BalancesCommands {
    #[command(subcommand)]
    command: BalancesSubcommand,
}

#[derive(clap::Subcommand)]
enum BalancesSubcommand {
    /// List all balances for a profile
    List {
        /// Profile ID
        profile_id: i64,
    },
    /// Get a specific balance
    Get {
        /// Profile ID
        profile_id: i64,
        /// Balance ID
        balance_id: i64,
    },
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: BalancesCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        BalancesSubcommand::List { profile_id } => {
            let balances = client.balances().list(profile_id, None).await?;
            output::print(&balances, format);
        }
        BalancesSubcommand::Get {
            profile_id,
            balance_id,
        } => {
            let balance = client.balances().get(profile_id, balance_id).await?;
            output::print(&balance, format);
        }
    }
    Ok(())
}
