//! Balance statement API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{BalanceStatement, StatementType};
use chrono::{DateTime, Utc};

/// Balance Statement API operations (read-only).
pub struct StatementsApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> StatementsApi<'a> {
    /// Get a balance statement in JSON format.
    ///
    /// # Arguments
    /// * `profile_id` - Profile ID
    /// * `balance_id` - Balance ID
    /// * `currency` - Currency code
    /// * `interval_start` - Start of the statement period
    /// * `interval_end` - End of the statement period (max ~469 days from start)
    /// * `statement_type` - COMPACT or FLAT
    /// * `locale` - Optional 2-character language code
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// use chrono::{TimeZone, Utc};
    /// use wise_client::models::StatementType;
    /// # let client = wise_client::ReadOnlyClient::new(wise_client::ClientConfig::with_token("test"))?;
    ///
    /// let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    /// let end = Utc.with_ymd_and_hms(2024, 3, 31, 23, 59, 59).unwrap();
    ///
    /// let statement = client.statements().get_json(
    ///     12345, 67890, "EUR", start, end, StatementType::Compact, None
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_json(
        &self,
        profile_id: i64,
        balance_id: i64,
        currency: &str,
        interval_start: DateTime<Utc>,
        interval_end: DateTime<Utc>,
        statement_type: StatementType,
        locale: Option<&str>,
    ) -> Result<BalanceStatement> {
        let type_str = match statement_type {
            StatementType::Compact => "COMPACT",
            StatementType::Flat => "FLAT",
        };

        let mut url = format!(
            "/v1/profiles/{}/balance-statements/{}/statement.json?currency={}&intervalStart={}&intervalEnd={}&type={}",
            profile_id,
            balance_id,
            currency,
            interval_start.format("%Y-%m-%dT%H:%M:%S%.3fZ"),
            interval_end.format("%Y-%m-%dT%H:%M:%S%.3fZ"),
            type_str
        );

        if let Some(loc) = locale {
            url.push_str(&format!("&statementLocale={}", loc));
        }

        self.client.get(&url).await
    }
}
