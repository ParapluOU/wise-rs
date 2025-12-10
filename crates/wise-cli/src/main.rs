//! Wise CLI - Command-line interface for the Wise Platform API.

use clap::{Parser, Subcommand};

mod commands;
mod config;
mod output;

use commands::{BalancesCommands, ProfilesCommands, RatesCommands, TransfersCommands, UserCommands};

#[derive(Parser)]
#[command(name = "wise")]
#[command(about = "Command-line interface for the Wise Platform API")]
#[command(version)]
pub struct Cli {
    /// Config file path
    #[arg(short, long, global = true)]
    pub config: Option<std::path::PathBuf>,

    /// Output format
    #[arg(short, long, global = true, default_value = "human")]
    pub output: output::OutputFormat,

    /// Use production environment (default: sandbox)
    #[arg(long, global = true)]
    pub production: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// User operations
    User(UserCommands),
    /// Profile operations
    Profiles(ProfilesCommands),
    /// Balance operations
    Balances(BalancesCommands),
    /// Transfer operations
    Transfers(TransfersCommands),
    /// Exchange rate operations
    Rates(RatesCommands),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let config = config::load_config(cli.config.as_deref())?;

    let client_config = wise_client::ClientConfig::with_token(&config.api_token);
    let client_config = if cli.production {
        client_config.production()
    } else {
        client_config.sandbox()
    };

    let client = wise_client::ReadOnlyClient::new(client_config)?;

    match cli.command {
        Commands::User(cmd) => commands::user::run(&client, cmd, cli.output).await?,
        Commands::Profiles(cmd) => commands::profiles::run(&client, cmd, cli.output).await?,
        Commands::Balances(cmd) => commands::balances::run(&client, cmd, cli.output).await?,
        Commands::Transfers(cmd) => commands::transfers::run(&client, cmd, cli.output).await?,
        Commands::Rates(cmd) => commands::rates::run(&client, cmd, cli.output).await?,
    }

    Ok(())
}
