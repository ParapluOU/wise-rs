//! Wise CLI - Command-line interface for the Wise Platform API.

use clap::{Parser, Subcommand};

mod commands;
mod config;
mod output;

use commands::{
    ActivitiesCommands, AddressesCommands, BalancesCommands, BankDetailsCommands,
    CardOrdersCommands, CardTransactionsCommands, CardsCommands, CasesCommands,
    CurrenciesCommands, DirectDebitsCommands, DisputesCommands, ProfilesCommands,
    QuotesCommands, RatesCommands, RecipientsCommands, StatementsCommands, TransfersCommands,
    UserCommands,
};

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
    /// Activity operations
    Activities(ActivitiesCommands),
    /// Address operations
    Addresses(AddressesCommands),
    /// Statement operations
    Statements(StatementsCommands),
    /// Bank details operations
    BankDetails(BankDetailsCommands),
    /// Card operations
    Cards(CardsCommands),
    /// Card order operations
    CardOrders(CardOrdersCommands),
    /// Card transaction operations
    CardTransactions(CardTransactionsCommands),
    /// Support case operations
    Cases(CasesCommands),
    /// Currency operations
    Currencies(CurrenciesCommands),
    /// Direct debit operations
    DirectDebits(DirectDebitsCommands),
    /// Dispute operations
    Disputes(DisputesCommands),
    /// Quote operations
    Quotes(QuotesCommands),
    /// Recipient operations
    Recipients(RecipientsCommands),
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
        Commands::Activities(cmd) => commands::activities::run(&client, cmd, cli.output).await?,
        Commands::Addresses(cmd) => commands::addresses::run(&client, cmd, cli.output).await?,
        Commands::Statements(cmd) => commands::statements::run(&client, cmd, cli.output).await?,
        Commands::BankDetails(cmd) => commands::bank_details::run(&client, cmd, cli.output).await?,
        Commands::Cards(cmd) => commands::cards::run(&client, cmd, cli.output).await?,
        Commands::CardOrders(cmd) => commands::card_orders::run(&client, cmd, cli.output).await?,
        Commands::CardTransactions(cmd) => {
            commands::card_transactions::run(&client, cmd, cli.output).await?
        }
        Commands::Cases(cmd) => commands::cases::run(&client, cmd, cli.output).await?,
        Commands::Currencies(cmd) => commands::currencies::run(&client, cmd, cli.output).await?,
        Commands::DirectDebits(cmd) => {
            commands::direct_debits::run(&client, cmd, cli.output).await?
        }
        Commands::Disputes(cmd) => commands::disputes::run(&client, cmd, cli.output).await?,
        Commands::Quotes(cmd) => commands::quotes::run(&client, cmd, cli.output).await?,
        Commands::Recipients(cmd) => commands::recipients::run(&client, cmd, cli.output).await?,
    }

    Ok(())
}
