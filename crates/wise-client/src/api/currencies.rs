//! Currency API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::Currency;

/// Currency API operations (read-only).
pub struct CurrenciesApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> CurrenciesApi<'a> {
    /// List all supported currencies.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// # let client = wise_client::ReadOnlyClient::new(wise_client::ClientConfig::with_token("test"))?;
    /// let currencies = client.currencies().list().await?;
    /// for currency in currencies {
    ///     println!("{}: {} ({})", currency.code, currency.name, currency.symbol);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<Vec<Currency>> {
        self.client.get("/v1/currencies").await
    }
}
