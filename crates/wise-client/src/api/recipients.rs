//! Recipient API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{CreateRecipient, Recipient};

/// Read-only Recipient API operations.
pub struct RecipientsApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> RecipientsApi<'a> {
    /// List recipient accounts.
    ///
    /// # Arguments
    /// * `profile_id` - Filter by profile ID (optional)
    /// * `currency` - Filter by currency code (optional)
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// # let client = wise_client::ReadOnlyClient::new(wise_client::ClientConfig::with_token("test"))?;
    /// let recipients = client.recipients().list(Some(12345), None).await?;
    /// for recipient in recipients {
    ///     println!("{}: {} ({})", recipient.id, recipient.account_holder_name, recipient.currency);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        profile_id: Option<i64>,
        currency: Option<&str>,
    ) -> Result<Vec<Recipient>> {
        let mut url = "/v2/accounts".to_string();
        let mut params = Vec::new();

        if let Some(pid) = profile_id {
            params.push(format!("profile={}", pid));
        }
        if let Some(c) = currency {
            params.push(format!("currency={}", c));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        self.client.get(&url).await
    }

    /// Get a specific recipient by ID.
    pub async fn get(&self, account_id: i64) -> Result<Recipient> {
        self.client
            .get(&format!("/v2/accounts/{}", account_id))
            .await
    }
}

/// Full Recipient API operations (includes write operations).
pub struct RecipientsApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> RecipientsApiMut<'a> {
    /// List recipient accounts.
    pub async fn list(
        &self,
        profile_id: Option<i64>,
        currency: Option<&str>,
    ) -> Result<Vec<Recipient>> {
        let mut url = "/v2/accounts".to_string();
        let mut params = Vec::new();

        if let Some(pid) = profile_id {
            params.push(format!("profile={}", pid));
        }
        if let Some(c) = currency {
            params.push(format!("currency={}", c));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        self.client.get(&url).await
    }

    /// Get a specific recipient by ID.
    pub async fn get(&self, account_id: i64) -> Result<Recipient> {
        self.client
            .get(&format!("/v2/accounts/{}", account_id))
            .await
    }

    /// Create a new recipient account.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// use wise_client::models::CreateRecipient;
    /// # let client = wise_client::FullClient::new(wise_client::ClientConfig::with_token("test"))?;
    ///
    /// let recipient = client.recipients().create(&CreateRecipient {
    ///     profile: 12345,
    ///     account_holder_name: "John Doe".to_string(),
    ///     currency: "EUR".to_string(),
    ///     account_type: "iban".to_string(),
    ///     details: serde_json::json!({
    ///         "IBAN": "DE89370400440532013000",
    ///         "legalType": "PRIVATE"
    ///     }),
    ///     refund: None,
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, request: &CreateRecipient) -> Result<Recipient> {
        self.client.post("/v1/accounts", request).await
    }

    /// Delete (deactivate) a recipient account.
    pub async fn delete(&self, account_id: i64) -> Result<()> {
        self.client
            .delete(&format!("/v2/accounts/{}", account_id))
            .await
    }
}
