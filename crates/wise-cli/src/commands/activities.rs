//! Activity command implementations.

use clap::Args;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct ActivitiesCommands {
    #[command(subcommand)]
    command: ActivitiesSubcommand,
}

#[derive(clap::Subcommand)]
enum ActivitiesSubcommand {
    /// List activities for a profile
    List {
        /// Profile ID
        profile_id: i64,
        /// Number of activities to return (1-100)
        #[arg(short, long)]
        size: Option<u32>,
    },
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: ActivitiesCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        ActivitiesSubcommand::List { profile_id, size } => {
            let mut params = wise_client::ListActivitiesParams::default();
            params.size = size;
            let activities = client.activities().list(profile_id, Some(params)).await?;
            output::print(&activities, format);
        }
    }
    Ok(())
}
