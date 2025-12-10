//! Transfer API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{CreateTransfer, FundTransfer, Transfer};

/// Read-only Transfer API operations.
pub struct TransfersApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> TransfersApi<'a> {
    /// List transfers with optional filters.
    ///
    /// # Arguments
    /// * `profile_id` - Filter by profile ID (optional)
    /// * `limit` - Maximum number of results
    /// * `offset` - Number of results to skip
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// # let client = wise_client::ReadOnlyClient::new(wise_client::ClientConfig::with_token("test"))?;
    /// let transfers = client.transfers().list(Some(12345), Some(10), None).await?;
    /// for transfer in transfers {
    ///     println!("Transfer {}: {} {} -> {} {}",
    ///         transfer.id,
    ///         transfer.source_value, transfer.source_currency,
    ///         transfer.target_value, transfer.target_currency
    ///     );
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        profile_id: Option<i64>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Transfer>> {
        let mut url = "/v1/transfers".to_string();
        let mut params = Vec::new();

        if let Some(pid) = profile_id {
            params.push(format!("profile={}", pid));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(o) = offset {
            params.push(format!("offset={}", o));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        self.client.get(&url).await
    }

    /// Get a specific transfer by ID.
    pub async fn get(&self, transfer_id: i64) -> Result<Transfer> {
        self.client
            .get(&format!("/v1/transfers/{}", transfer_id))
            .await
    }

    /// Get transfer delivery estimate.
    pub async fn get_delivery_estimate(&self, transfer_id: i64) -> Result<serde_json::Value> {
        self.client
            .get(&format!("/v1/delivery-estimates/{}", transfer_id))
            .await
    }
}

/// Full Transfer API operations (includes write operations).
pub struct TransfersApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> TransfersApiMut<'a> {
    /// List transfers with optional filters.
    pub async fn list(
        &self,
        profile_id: Option<i64>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Transfer>> {
        let mut url = "/v1/transfers".to_string();
        let mut params = Vec::new();

        if let Some(pid) = profile_id {
            params.push(format!("profile={}", pid));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(o) = offset {
            params.push(format!("offset={}", o));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        self.client.get(&url).await
    }

    /// Get a specific transfer by ID.
    pub async fn get(&self, transfer_id: i64) -> Result<Transfer> {
        self.client
            .get(&format!("/v1/transfers/{}", transfer_id))
            .await
    }

    /// Create a new transfer.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// use wise_client::models::{CreateTransfer, CreateTransferDetails};
    /// # let client = wise_client::FullClient::new(wise_client::ClientConfig::with_token("test"))?;
    ///
    /// let transfer = client.transfers().create(&CreateTransfer {
    ///     target_account: 12345,
    ///     quote_uuid: "quote-uuid-here".to_string(),
    ///     customer_transaction_id: uuid::Uuid::new_v4().to_string(),
    ///     details: CreateTransferDetails {
    ///         reference: "Payment for services".to_string(),
    ///         transfer_purpose: None,
    ///         source_of_funds: None,
    ///     },
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, request: &CreateTransfer) -> Result<Transfer> {
        self.client.post("/v1/transfers", request).await
    }

    /// Fund a transfer from a balance.
    ///
    /// # Arguments
    /// * `profile_id` - The profile ID
    /// * `transfer_id` - The transfer ID to fund
    /// * `request` - Funding details
    pub async fn fund(
        &self,
        profile_id: i64,
        transfer_id: i64,
        request: &FundTransfer,
    ) -> Result<serde_json::Value> {
        self.client
            .post(
                &format!(
                    "/v3/profiles/{}/transfers/{}/payments",
                    profile_id, transfer_id
                ),
                request,
            )
            .await
    }

    /// Cancel a transfer.
    pub async fn cancel(&self, transfer_id: i64) -> Result<Transfer> {
        self.client
            .put(&format!("/v1/transfers/{}/cancel", transfer_id), &())
            .await
    }

    /// Get transfer delivery estimate.
    pub async fn get_delivery_estimate(&self, transfer_id: i64) -> Result<serde_json::Value> {
        self.client
            .get(&format!("/v1/delivery-estimates/{}", transfer_id))
            .await
    }
}
