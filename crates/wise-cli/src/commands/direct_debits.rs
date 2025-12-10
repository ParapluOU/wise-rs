//! Direct debit command implementations.

use clap::Args;
use wise_client::models::DirectDebitType;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct DirectDebitsCommands {
    #[command(subcommand)]
    command: DirectDebitsSubcommand,
}

#[derive(clap::Subcommand)]
enum DirectDebitsSubcommand {
    /// List direct debit accounts for a profile
    List {
        /// Profile ID
        profile_id: i64,
        /// Direct debit type: ACH (for USD) or EFT (for CAD)
        #[arg(value_parser = parse_debit_type)]
        debit_type: DirectDebitType,
        /// Currency code (USD or CAD)
        currency: String,
    },
}

fn parse_debit_type(s: &str) -> Result<DirectDebitType, String> {
    match s.to_uppercase().as_str() {
        "ACH" => Ok(DirectDebitType::Ach),
        "EFT" => Ok(DirectDebitType::Eft),
        _ => Err("debit_type must be ACH or EFT".to_string()),
    }
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: DirectDebitsCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        DirectDebitsSubcommand::List {
            profile_id,
            debit_type,
            currency,
        } => {
            let accounts = client
                .direct_debits()
                .list(profile_id, debit_type, &currency)
                .await?;
            output::print(&accounts, format);
        }
    }
    Ok(())
}
