//! Exchange rate API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::Rate;
use chrono::{DateTime, Utc};

/// Exchange Rate API operations (read-only).
pub struct RatesApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> RatesApi<'a> {
    /// Get the current exchange rate between two currencies.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// # let client = wise_client::ReadOnlyClient::new(wise_client::ClientConfig::with_token("test"))?;
    /// let rate = client.rates().get("GBP", "USD").await?;
    /// println!("1 GBP = {} USD", rate.rate);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, source: &str, target: &str) -> Result<Rate> {
        let rates: Vec<Rate> = self
            .client
            .get(&format!("/v1/rates?source={}&target={}", source, target))
            .await?;

        rates.into_iter().next().ok_or_else(|| {
            crate::error::Error::Api {
                status: 404,
                message: format!("No rate found for {} -> {}", source, target),
                errors: vec![],
            }
        })
    }

    /// Get all available exchange rates.
    pub async fn list(&self) -> Result<Vec<Rate>> {
        self.client.get("/v1/rates").await
    }

    /// Get a historical exchange rate at a specific time.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// use chrono::{TimeZone, Utc};
    /// # let client = wise_client::ReadOnlyClient::new(wise_client::ClientConfig::with_token("test"))?;
    ///
    /// let time = Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();
    /// let rate = client.rates().at_time("GBP", "USD", time).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn at_time(&self, source: &str, target: &str, time: DateTime<Utc>) -> Result<Rate> {
        let time_str = time.format("%Y-%m-%dT%H:%M:%S+00:00").to_string();
        let rates: Vec<Rate> = self
            .client
            .get(&format!(
                "/v1/rates?source={}&target={}&time={}",
                source, target, time_str
            ))
            .await?;

        rates.into_iter().next().ok_or_else(|| {
            crate::error::Error::Api {
                status: 404,
                message: format!("No rate found for {} -> {} at {}", source, target, time),
                errors: vec![],
            }
        })
    }

    /// Get historical rate data over a time range.
    ///
    /// # Arguments
    /// * `source` - Source currency code
    /// * `target` - Target currency code
    /// * `from` - Start of time range
    /// * `to` - End of time range
    /// * `group` - Grouping interval ("day", "hour", "minute")
    pub async fn history(
        &self,
        source: &str,
        target: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        group: &str,
    ) -> Result<Vec<Rate>> {
        let from_str = from.format("%Y-%m-%dT%H:%M:%S+00:00").to_string();
        let to_str = to.format("%Y-%m-%dT%H:%M:%S+00:00").to_string();
        self.client
            .get(&format!(
                "/v1/rates?source={}&target={}&from={}&to={}&group={}",
                source, target, from_str, to_str, group
            ))
            .await
    }
}
