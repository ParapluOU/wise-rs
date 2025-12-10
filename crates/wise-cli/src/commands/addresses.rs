//! Address command implementations.

use clap::Args;
use wise_client::ReadOnlyClient;

use crate::output::{self, OutputFormat};

#[derive(Args)]
pub struct AddressesCommands {
    #[command(subcommand)]
    command: AddressesSubcommand,
}

#[derive(clap::Subcommand)]
enum AddressesSubcommand {
    /// List addresses for a profile
    List {
        /// Profile ID
        profile_id: i64,
    },
    /// Get an address by ID
    Get {
        /// Address ID
        address_id: i64,
    },
}

pub async fn run(
    client: &ReadOnlyClient,
    cmd: AddressesCommands,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd.command {
        AddressesSubcommand::List { profile_id } => {
            let addresses = client.addresses().list(profile_id).await?;
            output::print(&addresses, format);
        }
        AddressesSubcommand::Get { address_id } => {
            let address = client.addresses().get(address_id).await?;
            output::print(&address, format);
        }
    }
    Ok(())
}
