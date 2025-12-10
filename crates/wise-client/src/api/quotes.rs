//! Quote API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{CreateQuote, Quote};
use uuid::Uuid;

/// Read-only Quote API operations.
pub struct QuotesApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> QuotesApi<'a> {
    /// Get a specific quote by ID.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// # let client = wise_client::ReadOnlyClient::new(wise_client::ClientConfig::with_token("test"))?;
    /// let quote_id = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000")?;
    /// let quote = client.quotes().get(12345, quote_id).await?;
    /// println!("Quote rate: {}", quote.rate);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, profile_id: i64, quote_id: Uuid) -> Result<Quote> {
        self.client
            .get(&format!("/v3/profiles/{}/quotes/{}", profile_id, quote_id))
            .await
    }
}

/// Full Quote API operations (includes write operations).
pub struct QuotesApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> QuotesApiMut<'a> {
    /// Get a specific quote by ID.
    pub async fn get(&self, profile_id: i64, quote_id: Uuid) -> Result<Quote> {
        self.client
            .get(&format!("/v3/profiles/{}/quotes/{}", profile_id, quote_id))
            .await
    }

    /// Create a new quote.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// use rust_decimal_macros::dec;
    /// use wise_client::models::CreateQuote;
    /// # let client = wise_client::FullClient::new(wise_client::ClientConfig::with_token("test"))?;
    ///
    /// let quote = client.quotes().create(
    ///     12345, // profile_id
    ///     &CreateQuote::with_source_amount("GBP", "USD", dec!(100.00))
    /// ).await?;
    /// println!("Quote: {} {} -> {} {}",
    ///     quote.source_amount.unwrap_or_default(), quote.source_currency,
    ///     quote.target_amount.unwrap_or_default(), quote.target_currency
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, profile_id: i64, request: &CreateQuote) -> Result<Quote> {
        self.client
            .post(&format!("/v3/profiles/{}/quotes", profile_id), request)
            .await
    }

    /// Create an unauthenticated quote (no profile ID required).
    ///
    /// Useful for showing indicative rates before user authenticates.
    pub async fn create_unauthenticated(&self, request: &CreateQuote) -> Result<Quote> {
        self.client.post("/v3/quotes", request).await
    }

    /// Update an existing quote.
    ///
    /// Used to add a target account to an existing quote.
    pub async fn update(
        &self,
        profile_id: i64,
        quote_id: Uuid,
        request: &CreateQuote,
    ) -> Result<Quote> {
        self.client
            .patch(&format!("/v3/profiles/{}/quotes/{}", profile_id, quote_id), request)
            .await
    }
}
