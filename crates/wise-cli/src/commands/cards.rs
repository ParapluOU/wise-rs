//! Card command implementations.

use clap::Args;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct CardsCommands {
    #[command(subcommand)]
    command: CardsSubcommand,
}

#[derive(clap::Subcommand)]
enum CardsSubcommand {
    /// List cards for a profile
    List {
        /// Profile ID
        profile_id: i64,
        /// Page size (10-100)
        #[arg(long)]
        page_size: Option<u32>,
        /// Page number
        #[arg(long)]
        page: Option<u32>,
    },
    /// Get a card by token
    Get {
        /// Profile ID
        profile_id: i64,
        /// Card token
        card_token: String,
    },
    /// Get spending permissions for a card
    Permissions {
        /// Profile ID
        profile_id: i64,
        /// Card token
        card_token: String,
    },
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: CardsCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        CardsSubcommand::List {
            profile_id,
            page_size,
            page,
        } => {
            let cards = client.cards().list(profile_id, page_size, page).await?;
            output::print(&cards, format);
        }
        CardsSubcommand::Get {
            profile_id,
            card_token,
        } => {
            let card = client.cards().get(profile_id, &card_token).await?;
            output::print(&card, format);
        }
        CardsSubcommand::Permissions {
            profile_id,
            card_token,
        } => {
            let permissions = client.cards().get_permissions(profile_id, &card_token).await?;
            output::print(&permissions, format);
        }
    }
    Ok(())
}
