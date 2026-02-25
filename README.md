# wise-rs

A Rust workspace for the [Wise Platform API](https://docs.wise.com/api-docs/api-reference), providing:

- **wise-client** - Type-safe async HTTP client library
- **wise-cli** - Command-line interface
- **wise-mcp** - MCP server for AI assistant integration

> ### Looking for IT services?
> <img src="https://fromulo.com/codesociety.png" align="left" width="80" alt="CodeSociety">
>
> **[CodeSociety](https://codesocietyhub.com/)** is our consulting & contracting arm — specializing in
> **IT architecture**, **XML authoring systems**, **FontoXML integration**, and **TerminusDB consulting**.
> We build structured content platforms and data solutions that power digital publishing.
>
> **[Let's talk! &#8594;](https://codesocietyhub.com/contact.html)**

## Features

- Compile-time safety with separate `ReadOnlyClient` and `FullClient` types
- Comprehensive API coverage (23+ resource types)
- Both sandbox and production environments
- Flexible datetime handling for Wise API formats
- Secure token handling (never logged or exposed)

## Installation

```bash
# Clone the repository
git clone https://github.com/ParapluOU/wise-rs.git
cd wise-rs

# Build all crates
cargo build --release

# Install CLI globally (optional)
cargo install --path crates/wise-cli
```

## Configuration

### API Token

Get your API token from the [Wise Settings page](https://wise.com/settings/api-tokens).

The tools look for the token in this order:
1. `.secrets.env` file in current or parent directories
2. `WISE_API_KEY_RO` environment variable
3. `WISE_API_TOKEN` environment variable
4. Config file (`~/.config/paraplu/wise/config.toml`)

Example `.secrets.env`:
```env
WISE_API_KEY_RO=your-api-token-here
```

Example `config.toml`:
```toml
api_token = "your-api-token-here"
```

---

## wise-client (Library)

Type-safe async Rust client with compile-time read/write separation.

### Quick Start

```rust
use wise_client::{ClientConfig, ReadOnlyClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a read-only client (sandbox by default)
    let config = ClientConfig::with_token("your-api-token");
    let client = ReadOnlyClient::new(config)?;

    // Get authenticated user
    let user = client.user().get().await?;
    println!("Hello, {}!", user.name.unwrap_or_default());

    // List profiles
    let profiles = client.profiles().list().await?;
    for profile in profiles {
        println!("Profile: {} ({})", profile.id, profile.display_name());
    }

    // Get exchange rate
    let rate = client.rates().get("USD", "EUR").await?;
    println!("USD/EUR: {}", rate.rate);

    Ok(())
}
```

### Client Types

```rust
// Read-only client - only GET operations, safe for most use cases
let client = ReadOnlyClient::new(config)?;

// Full client - includes create, update, delete operations
let client = FullClient::new(config)?;

// Get read-only reference from full client
let read_only = client.as_read_only();
```

### Configuration Options

```rust
use std::time::Duration;
use wise_client::ClientConfig;

let config = ClientConfig::with_token("token")
    .sandbox()                           // Use sandbox (default)
    .production()                        // Use production
    .timeout(Duration::from_secs(60))    // Custom timeout
    .user_agent("my-app/1.0");           // Custom user agent
```

### Available APIs

| API | Description |
|-----|-------------|
| `client.user()` | Authenticated user info |
| `client.profiles()` | Profile management |
| `client.balances()` | Account balances |
| `client.transfers()` | Money transfers |
| `client.rates()` | Exchange rates |
| `client.currencies()` | Supported currencies |
| `client.recipients()` | Recipient accounts |
| `client.quotes()` | Transfer quotes |
| `client.activities()` | Activity feed |
| `client.addresses()` | Profile addresses |
| `client.statements()` | Balance statements |
| `client.bank_details()` | Receiving bank details |
| `client.cards()` | Card information |
| `client.card_orders()` | Card orders |
| `client.card_transactions()` | Card transactions |
| `client.cases()` | Support cases |
| `client.direct_debits()` | Direct debit accounts |
| `client.disputes()` | Dispute management |

### Error Handling

```rust
use wise_client::error::Error;

match client.user().get().await {
    Ok(user) => println!("User: {:?}", user),
    Err(Error::Api { status, message, .. }) => {
        eprintln!("API error {}: {}", status, message);
    }
    Err(Error::RateLimit { retry_after_secs }) => {
        eprintln!("Rate limited, retry after {:?}s", retry_after_secs);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## wise-cli (Command Line)

Human-friendly CLI for interacting with the Wise API.

### Usage

```bash
wise [OPTIONS] <COMMAND>

Options:
  -c, --config <PATH>     Config file path
  -o, --output <FORMAT>   Output format: human (default) or json
      --production        Use production environment (default: sandbox)
  -h, --help              Print help
  -V, --version           Print version
```

### Commands

```bash
# User info
wise user

# Profiles
wise profiles list
wise profiles get <profile_id>

# Balances
wise balances list <profile_id>
wise balances get <profile_id> <balance_id>

# Transfers
wise transfers list -p <profile_id> -l 10
wise transfers get <transfer_id>

# Exchange rates
wise rates get USD EUR
wise rates list

# Currencies
wise currencies list

# Recipients
wise recipients list <profile_id>
wise recipients get <account_id>

# Activities
wise activities list <profile_id> --size 20

# Addresses
wise addresses list <profile_id>
wise addresses get <address_id>

# Statements
wise statements get <profile_id> <balance_id> <currency> <start_date> <end_date>

# Bank details
wise bank-details list <profile_id>

# Cards
wise cards list <profile_id>
wise cards get <card_token>

# Card orders
wise card-orders list <profile_id>
wise card-orders get <order_id>

# Card transactions
wise card-transactions get <profile_id> <transaction_id>

# Support cases
wise cases get <case_id>
wise cases comments <case_id>

# Direct debits
wise direct-debits list <profile_id>

# Disputes
wise disputes list <profile_id>
wise disputes get <profile_id> <dispute_id>
wise disputes reasons <profile_id>

# Quotes
wise quotes get <profile_id> <quote_id>
```

### Examples

```bash
# Get user info as JSON
wise -o json user

# List production profiles
wise --production profiles list

# Get EUR balances
wise balances list 12345 | grep EUR

# Export transfers to file
wise -o json transfers list -p 12345 > transfers.json
```

---

## wise-mcp (MCP Server)

Model Context Protocol server for AI assistant integration (e.g., Claude).

### Configuration

Set environment variables:

```bash
export WISE_API_TOKEN="your-api-token"
export WISE_READ_ONLY="true"       # Optional, default: true
export WISE_PRODUCTION="false"     # Optional, default: false (sandbox)
```

### Running

```bash
# Run the MCP server (uses stdio transport)
wise-mcp
```

### Claude Desktop Integration

Add to your Claude Desktop config (`~/.config/claude/claude_desktop_config.json` on Linux/macOS or `%APPDATA%\Claude\claude_desktop_config.json` on Windows):

```json
{
  "mcpServers": {
    "wise": {
      "command": "/path/to/wise-mcp",
      "env": {
        "WISE_API_TOKEN": "your-api-token",
        "WISE_PRODUCTION": "true"
      }
    }
  }
}
```

### Available Tools

The MCP server exposes 30+ tools for AI assistants:

**Account & Profile**
- `get_user` - Get authenticated user
- `list_profiles` - List all profiles
- `get_profile` - Get profile by ID

**Balances**
- `list_balances` - List balances for profile
- `get_balance` - Get specific balance

**Transfers & Rates**
- `list_transfers` - List transfers
- `get_transfer` - Get transfer details
- `get_rate` - Get exchange rate
- `list_currencies` - List currencies

**Recipients & Quotes**
- `list_recipients` - List recipients
- `get_recipient` - Get recipient
- `get_quote` - Get quote

**Activities & Statements**
- `list_activities` - List activities
- `get_statement` - Get balance statement

**Cards**
- `list_cards` - List cards
- `get_card` - Get card details
- `list_card_orders` - List card orders
- `get_card_transaction` - Get transaction

**And more:** addresses, bank details, cases, direct debits, disputes...

---

## Development

```bash
# Run tests
cargo test --workspace

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace -- -D warnings

# Build documentation
cargo doc --workspace --open
```

## Environment

| Variable | Description | Default |
|----------|-------------|---------|
| `WISE_API_KEY_RO` | Read-only API token | - |
| `WISE_API_TOKEN` | API token (fallback) | - |
| `WISE_READ_ONLY` | MCP read-only mode | `true` |
| `WISE_PRODUCTION` | Use production API | `false` |

## License

MIT

## Links

- [Wise Platform API Documentation](https://docs.wise.com/api-docs/api-reference)
- [Wise API Sandbox](https://sandbox.wise.com)
- [Get API Token](https://wise.com/settings/api-tokens)
