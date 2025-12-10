//! Balance API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::Balance;

/// Read-only Balance API operations.
pub struct BalancesApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> BalancesApi<'a> {
    /// List all balances for a profile.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// # let client = wise_client::ReadOnlyClient::new(wise_client::ClientConfig::with_token("test"))?;
    /// let balances = client.balances().list(12345).await?;
    /// for balance in balances {
    ///     println!("{}: {}", balance.currency, balance.amount);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, profile_id: i64) -> Result<Vec<Balance>> {
        self.client
            .get(&format!("/v4/profiles/{}/balances", profile_id))
            .await
    }

    /// Get a specific balance by ID.
    pub async fn get(&self, profile_id: i64, balance_id: i64) -> Result<Balance> {
        self.client
            .get(&format!("/v4/profiles/{}/balances/{}", profile_id, balance_id))
            .await
    }
}

/// Full Balance API operations (includes write operations).
pub struct BalancesApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> BalancesApiMut<'a> {
    /// List all balances for a profile.
    pub async fn list(&self, profile_id: i64) -> Result<Vec<Balance>> {
        self.client
            .get(&format!("/v4/profiles/{}/balances", profile_id))
            .await
    }

    /// Get a specific balance by ID.
    pub async fn get(&self, profile_id: i64, balance_id: i64) -> Result<Balance> {
        self.client
            .get(&format!("/v4/profiles/{}/balances/{}", profile_id, balance_id))
            .await
    }

    // Write operations would go here (create, close, convert, etc.)
}
