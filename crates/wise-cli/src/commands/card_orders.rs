//! Card order command implementations.

use clap::Args;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct CardOrdersCommands {
    #[command(subcommand)]
    command: CardOrdersSubcommand,
}

#[derive(clap::Subcommand)]
enum CardOrdersSubcommand {
    /// List card orders for a profile
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
    /// Get a card order by ID
    Get {
        /// Profile ID
        profile_id: i64,
        /// Card order ID
        card_order_id: i64,
    },
    /// Get available card programs
    Availability {
        /// Profile ID
        profile_id: i64,
    },
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: CardOrdersCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        CardOrdersSubcommand::List {
            profile_id,
            page_size,
            page,
        } => {
            let orders = client.card_orders().list(profile_id, page_size, page).await?;
            output::print(&orders, format);
        }
        CardOrdersSubcommand::Get {
            profile_id,
            card_order_id,
        } => {
            let order = client.card_orders().get(profile_id, card_order_id).await?;
            output::print(&order, format);
        }
        CardOrdersSubcommand::Availability { profile_id } => {
            let availability = client.card_orders().get_availability(profile_id).await?;
            output::print(&availability, format);
        }
    }
    Ok(())
}
