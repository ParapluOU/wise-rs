//! Bulk settlement API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::SettlementJournal;

/// Settlement API operations (write-only).
pub struct SettlementsApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> SettlementsApiMut<'a> {
    /// Send a settlement journal.
    ///
    /// This endpoint requires a client credentials token.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// use wise_client::models::{SettlementJournal, SettlementTransfer};
    /// use rust_decimal_macros::dec;
    /// # let client = wise_client::FullClient::new(wise_client::ClientConfig::with_token("test"))?;
    ///
    /// let journal = SettlementJournal::new(
    ///     "TPFB190322",
    ///     "2024-03-22T23:59:59-05:00",
    ///     vec![
    ///         SettlementTransfer {
    ///             id: 125678,
    ///             date: "2024-03-22T10:00:12-05:00".to_string(),
    ///             source_amount: dec!(23.24),
    ///             source_currency: "USD".to_string(),
    ///             customer_name: "Joe Bloggs".to_string(),
    ///             partner_reference: "11111".to_string(),
    ///             comment: Some("Extra Data".to_string()),
    ///             exchange_rate: None,
    ///         },
    ///     ],
    /// );
    ///
    /// client.settlements().send(&journal).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(&self, journal: &SettlementJournal) -> Result<()> {
        let _: serde_json::Value = self.client.post("/v1/settlements", journal).await?;
        Ok(())
    }
}
