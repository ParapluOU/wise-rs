//! Card transaction API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::CardTransaction;

/// Card Transaction API operations (read-only).
pub struct CardTransactionsApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> CardTransactionsApi<'a> {
    /// Get a card transaction by ID.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// # let client = wise_client::ReadOnlyClient::new(wise_client::ClientConfig::with_token("test"))?;
    /// let tx = client.card_transactions().get(12345, "342671").await?;
    /// println!("Transaction: {} - {:?}", tx.transaction_type, tx.state);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, profile_id: i64, transaction_id: &str) -> Result<CardTransaction> {
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/cards/transactions/{}",
                profile_id, transaction_id
            ))
            .await
    }
}
