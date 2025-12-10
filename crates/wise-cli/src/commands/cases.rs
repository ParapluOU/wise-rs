//! Partner cases command implementations.

use clap::Args;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct CasesCommands {
    #[command(subcommand)]
    command: CasesSubcommand,
}

#[derive(clap::Subcommand)]
enum CasesSubcommand {
    /// Get a partner case by ID
    Get {
        /// Case ID
        case_id: i64,
    },
    /// Get comments for a partner case
    Comments {
        /// Case ID
        case_id: i64,
    },
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: CasesCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        CasesSubcommand::Get { case_id } => {
            let case = client.cases().get(case_id).await?;
            output::print(&case, format);
        }
        CasesSubcommand::Comments { case_id } => {
            let comments = client.cases().get_comments(case_id).await?;
            output::print(&comments, format);
        }
    }
    Ok(())
}
