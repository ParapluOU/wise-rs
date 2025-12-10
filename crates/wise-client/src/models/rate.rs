//! Exchange rate models.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Exchange rate.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rate {
    /// Source currency code
    pub source: String,
    /// Target currency code
    pub target: String,
    /// Exchange rate
    pub rate: Decimal,
    /// Timestamp of the rate
    pub time: DateTime<Utc>,
}

impl Rate {
    /// Convert an amount from source to target currency.
    pub fn convert(&self, amount: Decimal) -> Decimal {
        amount * self.rate
    }

    /// Get the inverse rate (target to source).
    pub fn inverse(&self) -> Decimal {
        Decimal::ONE / self.rate
    }
}
