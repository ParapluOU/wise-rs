//! Recipient command implementations.

use clap::Args;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct RecipientsCommands {
    #[command(subcommand)]
    command: RecipientsSubcommand,
}

#[derive(clap::Subcommand)]
enum RecipientsSubcommand {
    /// List recipients for a profile
    List {
        /// Profile ID
        profile_id: i64,
        /// Currency filter
        #[arg(short, long)]
        currency: Option<String>,
    },
    /// Get a recipient by ID
    Get {
        /// Account ID
        account_id: i64,
    },
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: RecipientsCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        RecipientsSubcommand::List {
            profile_id,
            currency,
        } => {
            let recipients = client
                .recipients()
                .list(Some(profile_id), currency.as_deref())
                .await?;
            output::print(&recipients, format);
        }
        RecipientsSubcommand::Get { account_id } => {
            let recipient = client.recipients().get(account_id).await?;
            output::print(&recipient, format);
        }
    }
    Ok(())
}
