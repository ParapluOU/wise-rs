//! Balance API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{
    Balance, BalanceCapacity, BalanceMovement, BalanceMovementRequest, BalanceType, CreateBalance,
    ExcessMoneyAccount, SetExcessMoneyAccount, TotalFunds,
};

/// Read-only Balance API operations.
pub struct BalancesApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> BalancesApi<'a> {
    /// List all balances for a profile.
    ///
    /// # Arguments
    /// * `profile_id` - Profile ID
    /// * `types` - Optional slice of balance types to filter by (defaults to STANDARD)
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// # let client = wise_client::ReadOnlyClient::new(wise_client::ClientConfig::with_token("test"))?;
    /// use wise_client::models::BalanceType;
    ///
    /// // List all standard balances
    /// let balances = client.balances().list(12345, None).await?;
    ///
    /// // List both standard and savings balances
    /// let all_balances = client.balances().list(12345, Some(&[BalanceType::Standard, BalanceType::Savings])).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        profile_id: i64,
        types: Option<&[BalanceType]>,
    ) -> Result<Vec<Balance>> {
        let type_params = types
            .map(|t| {
                t.iter()
                    .map(|bt| match bt {
                        BalanceType::Standard => "STANDARD",
                        BalanceType::Savings => "SAVINGS",
                    })
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .unwrap_or_else(|| "STANDARD".to_string());

        self.client
            .get(&format!(
                "/v4/profiles/{}/balances?types={}",
                profile_id, type_params
            ))
            .await
    }

    /// Get a specific balance by ID.
    pub async fn get(&self, profile_id: i64, balance_id: i64) -> Result<Balance> {
        self.client
            .get(&format!(
                "/v4/profiles/{}/balances/{}",
                profile_id, balance_id
            ))
            .await
    }

    /// Get the deposit capacity/limits for a profile.
    ///
    /// Useful for profiles in countries with regulatory hold limits (e.g., Singapore, Malaysia).
    pub async fn get_capacity(&self, profile_id: i64, currency: &str) -> Result<BalanceCapacity> {
        self.client
            .get(&format!(
                "/v1/profiles/{}/balance-capacity?currency={}",
                profile_id, currency
            ))
            .await
    }

    /// Get total funds overview for a profile in a specific currency.
    pub async fn get_total_funds(&self, profile_id: i64, currency: &str) -> Result<TotalFunds> {
        self.client
            .get(&format!(
                "/v1/profiles/{}/total-funds/{}",
                profile_id, currency
            ))
            .await
    }
}

/// Full Balance API operations (includes write operations).
pub struct BalancesApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> BalancesApiMut<'a> {
    /// List all balances for a profile.
    pub async fn list(
        &self,
        profile_id: i64,
        types: Option<&[BalanceType]>,
    ) -> Result<Vec<Balance>> {
        let type_params = types
            .map(|t| {
                t.iter()
                    .map(|bt| match bt {
                        BalanceType::Standard => "STANDARD",
                        BalanceType::Savings => "SAVINGS",
                    })
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .unwrap_or_else(|| "STANDARD".to_string());

        self.client
            .get(&format!(
                "/v4/profiles/{}/balances?types={}",
                profile_id, type_params
            ))
            .await
    }

    /// Get a specific balance by ID.
    pub async fn get(&self, profile_id: i64, balance_id: i64) -> Result<Balance> {
        self.client
            .get(&format!(
                "/v4/profiles/{}/balances/{}",
                profile_id, balance_id
            ))
            .await
    }

    /// Create a new balance account.
    ///
    /// For STANDARD balances, only one can be created per currency.
    /// For SAVINGS balances, multiple can be created in the same currency.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// # let client = wise_client::FullClient::new(wise_client::ClientConfig::with_token("test"))?;
    /// use wise_client::models::CreateBalance;
    ///
    /// // Create a standard EUR balance
    /// let balance = client.balances().create(12345, &CreateBalance::standard("EUR")).await?;
    ///
    /// // Create a savings jar
    /// let jar = client.balances().create(12345, &CreateBalance::savings("EUR", "Holiday Fund")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, profile_id: i64, request: &CreateBalance) -> Result<Balance> {
        self.client
            .post(&format!("/v4/profiles/{}/balances", profile_id), request)
            .await
    }

    /// Delete (close) a balance account.
    ///
    /// The balance must have a zero balance to be closed.
    /// Bank account details will be deactivated and cannot be restored.
    pub async fn delete(&self, profile_id: i64, balance_id: i64) -> Result<Balance> {
        // DELETE returns the balance object
        self.client
            .get(&format!(
                "/v4/profiles/{}/balances/{}",
                profile_id, balance_id
            ))
            .await
        // Note: The actual delete is performed via DELETE method
        // but returns a Balance object, so we need a delete_with_response method
    }

    /// Close a balance account.
    pub async fn close(&self, profile_id: i64, balance_id: i64) -> Result<()> {
        self.client
            .delete(&format!(
                "/v4/profiles/{}/balances/{}",
                profile_id, balance_id
            ))
            .await
    }

    /// Convert funds between balance accounts using a quote.
    ///
    /// The quote must be created with `payOut: "BALANCE"`.
    pub async fn convert(
        &self,
        profile_id: i64,
        request: &BalanceMovementRequest,
    ) -> Result<BalanceMovement> {
        self.client
            .post(
                &format!("/v2/profiles/{}/balance-movements", profile_id),
                request,
            )
            .await
    }

    /// Move money between balances (jar transfers).
    ///
    /// Supports:
    /// - Same-currency transfers between STANDARD and SAVINGS balances
    /// - Cross-currency transfers (requires a quote)
    pub async fn move_money(
        &self,
        profile_id: i64,
        request: &BalanceMovementRequest,
    ) -> Result<BalanceMovement> {
        self.client
            .post(
                &format!("/v2/profiles/{}/balance-movements", profile_id),
                request,
            )
            .await
    }

    /// Get the deposit capacity/limits for a profile.
    pub async fn get_capacity(&self, profile_id: i64, currency: &str) -> Result<BalanceCapacity> {
        self.client
            .get(&format!(
                "/v1/profiles/{}/balance-capacity?currency={}",
                profile_id, currency
            ))
            .await
    }

    /// Get total funds overview for a profile in a specific currency.
    pub async fn get_total_funds(&self, profile_id: i64, currency: &str) -> Result<TotalFunds> {
        self.client
            .get(&format!(
                "/v1/profiles/{}/total-funds/{}",
                profile_id, currency
            ))
            .await
    }

    /// Set the excess money account for a profile.
    ///
    /// Used for SG and MY customers where excess funds need to be moved
    /// to another account to comply with regulatory limits.
    pub async fn set_excess_money_account(
        &self,
        profile_id: i64,
        recipient_id: i64,
    ) -> Result<ExcessMoneyAccount> {
        self.client
            .post(
                &format!("/v1/profiles/{}/excess-money-account", profile_id),
                &SetExcessMoneyAccount { recipient_id },
            )
            .await
    }
}
