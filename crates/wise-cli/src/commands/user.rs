//! User command implementations.

use clap::Args;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct UserCommands {
    #[command(subcommand)]
    command: Option<UserSubcommand>,
}

#[derive(clap::Subcommand)]
enum UserSubcommand {
    /// Get current user info (default)
    Get,
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: UserCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command.unwrap_or(UserSubcommand::Get) {
        UserSubcommand::Get => {
            let user = client.user().get().await?;
            output::print(&user, format);
        }
    }
    Ok(())
}
