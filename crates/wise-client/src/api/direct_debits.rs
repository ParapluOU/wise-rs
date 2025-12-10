//! Direct debit account API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{CreateDirectDebitAccountRequest, DirectDebitAccount, DirectDebitType};

/// Read-only Direct Debit API operations.
pub struct DirectDebitsApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> DirectDebitsApi<'a> {
    /// List direct debit accounts for a profile.
    ///
    /// # Arguments
    /// * `profile_id` - Profile ID
    /// * `debit_type` - Payment type (ACH for USD, EFT for CAD)
    /// * `currency` - Currency code (USD or CAD)
    pub async fn list(
        &self,
        profile_id: i64,
        debit_type: DirectDebitType,
        currency: &str,
    ) -> Result<Vec<DirectDebitAccount>> {
        let type_str = match debit_type {
            DirectDebitType::Ach => "ACH",
            DirectDebitType::Eft => "EFT",
        };
        self.client
            .get(&format!(
                "/v1/profiles/{}/direct-debit-accounts?type={}&currency={}",
                profile_id, type_str, currency
            ))
            .await
    }
}

/// Full Direct Debit API operations (includes write operations).
pub struct DirectDebitsApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> DirectDebitsApiMut<'a> {
    /// List direct debit accounts for a profile.
    pub async fn list(
        &self,
        profile_id: i64,
        debit_type: DirectDebitType,
        currency: &str,
    ) -> Result<Vec<DirectDebitAccount>> {
        let type_str = match debit_type {
            DirectDebitType::Ach => "ACH",
            DirectDebitType::Eft => "EFT",
        };
        self.client
            .get(&format!(
                "/v1/profiles/{}/direct-debit-accounts?type={}&currency={}",
                profile_id, type_str, currency
            ))
            .await
    }

    /// Create a direct debit account.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// use wise_client::models::{CreateDirectDebitAccountRequest, BankAccountType};
    /// # let client = wise_client::FullClient::new(wise_client::ClientConfig::with_token("test"))?;
    ///
    /// // Create a USD ACH direct debit account
    /// let account = client.direct_debits().create(
    ///     12345,
    ///     &CreateDirectDebitAccountRequest::ach(
    ///         "000000000",
    ///         "0000000000",
    ///         BankAccountType::Checking,
    ///     ),
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        profile_id: i64,
        request: &CreateDirectDebitAccountRequest,
    ) -> Result<DirectDebitAccount> {
        self.client
            .post(
                &format!("/v1/profiles/{}/direct-debit-accounts", profile_id),
                request,
            )
            .await
    }
}
