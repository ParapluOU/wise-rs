//! Quote models.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Exchange rate quote.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    /// Quote ID (UUID)
    pub id: Uuid,
    /// Source currency code
    pub source_currency: String,
    /// Target currency code
    pub target_currency: String,
    /// Source amount (if specified)
    pub source_amount: Option<Decimal>,
    /// Target amount (if specified)
    pub target_amount: Option<Decimal>,
    /// Exchange rate
    pub rate: Decimal,
    /// Creation timestamp
    pub created_time: Option<DateTime<Utc>>,
    /// Expiration timestamp
    pub expiration_time: Option<DateTime<Utc>>,
    /// User ID
    pub user: Option<i64>,
    /// Profile ID
    pub profile: Option<i64>,
    /// Rate type
    pub rate_type: Option<RateType>,
    /// Payment options
    pub payment_options: Option<Vec<PaymentOption>>,
    /// Transfer status
    pub status: Option<QuoteStatus>,
}

/// Rate type for quotes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateType {
    /// Fixed rate (locked)
    Fixed,
    /// Floating rate (may change)
    Floating,
}

/// Quote status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuoteStatus {
    /// Quote is pending
    Pending,
    /// Quote is accepted
    Accepted,
    /// Quote has expired
    Expired,
}

/// Payment option for a quote.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentOption {
    /// Payment type
    pub pay_in: String,
    /// Payout method
    pub pay_out: String,
    /// Source amount for this option
    pub source_amount: Decimal,
    /// Target amount for this option
    pub target_amount: Decimal,
    /// Payment type fee
    pub payment_type_fee: Option<Decimal>,
    /// Total fee
    pub fee: Fee,
    /// Whether this is the disabled option
    pub disabled: Option<bool>,
}

/// Fee breakdown.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fee {
    /// Total fee amount
    pub total: Decimal,
    /// Fee currency
    pub currency: Option<String>,
}

/// Request to create a quote.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuote {
    /// Source currency code
    pub source_currency: String,
    /// Target currency code
    pub target_currency: String,
    /// Source amount (specify either source or target amount)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_amount: Option<Decimal>,
    /// Target amount (specify either source or target amount)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_amount: Option<Decimal>,
    /// Target recipient account ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account: Option<i64>,
    /// Payment metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_out: Option<String>,
    /// Preferred pay-in method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_pay_in: Option<String>,
}

impl CreateQuote {
    /// Create a quote request with source amount.
    pub fn with_source_amount(
        source_currency: impl Into<String>,
        target_currency: impl Into<String>,
        source_amount: Decimal,
    ) -> Self {
        Self {
            source_currency: source_currency.into(),
            target_currency: target_currency.into(),
            source_amount: Some(source_amount),
            target_amount: None,
            target_account: None,
            pay_out: None,
            preferred_pay_in: None,
        }
    }

    /// Create a quote request with target amount.
    pub fn with_target_amount(
        source_currency: impl Into<String>,
        target_currency: impl Into<String>,
        target_amount: Decimal,
    ) -> Self {
        Self {
            source_currency: source_currency.into(),
            target_currency: target_currency.into(),
            source_amount: None,
            target_amount: Some(target_amount),
            target_account: None,
            pay_out: None,
            preferred_pay_in: None,
        }
    }

    /// Set the target account.
    pub fn target_account(mut self, account_id: i64) -> Self {
        self.target_account = Some(account_id);
        self
    }
}
