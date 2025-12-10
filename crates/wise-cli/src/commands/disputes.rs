//! Disputes command implementations.

use clap::Args;
use wise_client::models::DisputeStatus;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct DisputesCommands {
    #[command(subcommand)]
    command: DisputesSubcommand,
}

#[derive(clap::Subcommand)]
enum DisputesSubcommand {
    /// List disputes for a profile
    List {
        /// Profile ID
        profile_id: i64,
        /// Filter by status: ACTIVE or CLOSED
        #[arg(long, value_parser = parse_status)]
        status: Option<DisputeStatus>,
        /// Page size (10-100)
        #[arg(long)]
        page_size: Option<u32>,
        /// Page number
        #[arg(long)]
        page: Option<u32>,
    },
    /// Get a dispute by ID
    Get {
        /// Profile ID
        profile_id: i64,
        /// Dispute ID
        dispute_id: String,
    },
    /// Get available dispute reasons
    Reasons {
        /// Profile ID
        profile_id: i64,
    },
}

fn parse_status(s: &str) -> Result<DisputeStatus, String> {
    match s.to_uppercase().as_str() {
        "ACTIVE" => Ok(DisputeStatus::Active),
        "CLOSED" => Ok(DisputeStatus::Closed),
        _ => Err("status must be ACTIVE or CLOSED".to_string()),
    }
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: DisputesCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        DisputesSubcommand::List {
            profile_id,
            status,
            page_size,
            page,
        } => {
            let disputes = client
                .disputes()
                .list(profile_id, status, None, page_size, page)
                .await?;
            output::print(&disputes, format);
        }
        DisputesSubcommand::Get {
            profile_id,
            dispute_id,
        } => {
            let dispute = client.disputes().get(profile_id, &dispute_id).await?;
            output::print(&dispute, format);
        }
        DisputesSubcommand::Reasons { profile_id } => {
            let reasons = client.disputes().get_reasons(profile_id).await?;
            output::print(&reasons, format);
        }
    }
    Ok(())
}
