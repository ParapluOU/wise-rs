//! Balance models.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Balance account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    /// Balance ID
    pub id: i64,
    /// Currency code (e.g., "USD", "EUR")
    pub currency: String,
    /// Balance type
    #[serde(rename = "type")]
    pub balance_type: BalanceType,
    /// Custom name for the balance
    pub name: Option<String>,
    /// Balance amount
    pub amount: Money,
    /// Reserved amount (pending transactions)
    pub reserved_amount: Option<Money>,
    /// Cash amount available
    pub cash_amount: Option<Money>,
    /// Total worth including investments
    pub total_worth: Option<Money>,
    /// Creation timestamp
    pub creation_time: Option<DateTime<Utc>>,
    /// Last modification timestamp
    pub modification_time: Option<DateTime<Utc>>,
    /// Whether the balance is visible
    pub visible: Option<bool>,
}

/// Type of balance account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BalanceType {
    /// Standard balance
    Standard,
    /// Savings balance
    Savings,
}

/// Monetary amount with currency.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Money {
    /// Numeric value
    pub value: Decimal,
    /// Currency code
    pub currency: String,
}

impl Money {
    /// Creates a new Money instance.
    pub fn new(value: Decimal, currency: impl Into<String>) -> Self {
        Self {
            value,
            currency: currency.into(),
        }
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.currency)
    }
}
