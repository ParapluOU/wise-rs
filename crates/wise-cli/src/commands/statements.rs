//! Statement command implementations.

use chrono::{DateTime, Utc};
use clap::Args;
use wise_client::models::StatementType;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct StatementsCommands {
    #[command(subcommand)]
    command: StatementsSubcommand,
}

#[derive(clap::Subcommand)]
enum StatementsSubcommand {
    /// Get a balance statement
    Get {
        /// Profile ID
        profile_id: i64,
        /// Balance ID
        balance_id: i64,
        /// Currency code
        currency: String,
        /// Start date (ISO 8601 format)
        interval_start: String,
        /// End date (ISO 8601 format)
        interval_end: String,
    },
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: StatementsCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        StatementsSubcommand::Get {
            profile_id,
            balance_id,
            currency,
            interval_start,
            interval_end,
        } => {
            let start: DateTime<Utc> = interval_start.parse()?;
            let end: DateTime<Utc> = interval_end.parse()?;
            let statement = client
                .statements()
                .get_json(
                    profile_id,
                    balance_id,
                    &currency,
                    start,
                    end,
                    StatementType::Compact,
                    None,
                )
                .await?;
            output::print(&statement, format);
        }
    }
    Ok(())
}
