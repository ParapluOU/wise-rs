//! Profile command implementations.

use clap::Args;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct ProfilesCommands {
    #[command(subcommand)]
    command: ProfilesSubcommand,
}

#[derive(clap::Subcommand)]
enum ProfilesSubcommand {
    /// List all profiles
    List,
    /// Get a specific profile
    Get {
        /// Profile ID
        id: i64,
    },
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: ProfilesCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        ProfilesSubcommand::List => {
            let profiles = client.profiles().list().await?;
            output::print(&profiles, format);
        }
        ProfilesSubcommand::Get { id } => {
            let profile = client.profiles().get(id).await?;
            output::print(&profile, format);
        }
    }
    Ok(())
}
