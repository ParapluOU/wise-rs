//! # wise-client
//!
//! Async Rust client for the [Wise Platform API](https://docs.wise.com/api-reference).
//!
//! ## Features
//!
//! - **Type-safe**: Strongly typed models for all API resources
//! - **Async**: Built on `reqwest` and `tokio` for async operations
//! - **Read-only mode**: `ReadOnlyClient` only exposes GET endpoints for safety
//! - **Full access**: `FullClient` includes all operations including transfers
//!
//! ## Quick Start
//!
//! ```no_run
//! use wise_client::{ReadOnlyClient, ClientConfig};
//!
//! #[tokio::main]
//! async fn main() -> wise_client::error::Result<()> {
//!     // Create a read-only client (safe for exploration)
//!     let client = ReadOnlyClient::new(
//!         ClientConfig::with_token("your-api-token").sandbox()
//!     )?;
//!
//!     // Get user info
//!     let user = client.user().get().await?;
//!     println!("Logged in as: {:?}", user.name);
//!
//!     // List profiles
//!     let profiles = client.profiles().list().await?;
//!     for profile in profiles {
//!         println!("Profile: {} ({})", profile.display_name(), profile.id);
//!     }
//!
//!     // Get exchange rates
//!     let rate = client.rates().get("GBP", "USD").await?;
//!     println!("1 GBP = {} USD", rate.rate);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Full Access (for transfers)
//!
//! ```no_run
//! use wise_client::{FullClient, ClientConfig};
//! use wise_client::models::{CreateQuote, CreateTransfer, CreateTransferDetails};
//! use rust_decimal_macros::dec;
//!
//! # async fn example() -> wise_client::error::Result<()> {
//! let client = FullClient::new(
//!     ClientConfig::with_token("your-api-token").sandbox()
//! )?;
//!
//! // Get profile
//! let profiles = client.profiles().list().await?;
//! let profile_id = profiles[0].id;
//!
//! // Create a quote
//! let quote = client.quotes().create(
//!     profile_id,
//!     &CreateQuote::with_source_amount("GBP", "USD", dec!(100.00))
//! ).await?;
//!
//! println!("Quote: {} GBP -> {} USD at rate {}",
//!     quote.source_amount.unwrap_or_default(),
//!     quote.target_amount.unwrap_or_default(),
//!     quote.rate
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Environment
//!
//! By default, the client uses the **sandbox** environment. Use `.production()`
//! to switch to the production API:
//!
//! ```no_run
//! use wise_client::{ReadOnlyClient, ClientConfig};
//!
//! # fn example() -> wise_client::error::Result<()> {
//! let client = ReadOnlyClient::new(
//!     ClientConfig::with_token("your-api-token").production()
//! )?;
//! # Ok(())
//! # }
//! ```

mod api;
mod client;
mod config;
pub mod error;
pub mod models;

pub use client::{FullClient, ReadOnlyClient, ReadOnlyClientRef};
pub use config::{AuthConfig, ClientConfig, Environment};
