//! Bank details command implementations.

use clap::Args;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct BankDetailsCommands {
    #[command(subcommand)]
    command: BankDetailsSubcommand,
}

#[derive(clap::Subcommand)]
enum BankDetailsSubcommand {
    /// List bank account details for a profile
    List {
        /// Profile ID
        profile_id: i64,
    },
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: BankDetailsCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        BankDetailsSubcommand::List { profile_id } => {
            let details = client.bank_details().list(profile_id).await?;
            output::print(&details, format);
        }
    }
    Ok(())
}
