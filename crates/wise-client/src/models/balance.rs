//! Balance models.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Emoji icon for the balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Investment state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investment_state: Option<InvestmentState>,
    /// Balance amount (available for transfers)
    pub amount: Money,
    /// Reserved amount (pending transactions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_amount: Option<Money>,
    /// Cash amount available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_amount: Option<Money>,
    /// Total worth including investments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_worth: Option<Money>,
    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<DateTime<Utc>>,
    /// Last modification timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<DateTime<Utc>>,
    /// Whether the balance is visible
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

/// Type of balance account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BalanceType {
    /// Standard balance
    Standard,
    /// Savings balance (jar)
    Savings,
}

/// Investment state of a balance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvestmentState {
    /// Not invested
    NotInvested,
    /// Currently invested
    Invested,
    /// In process of divesting
    Divesting,
    /// Unknown state
    Unknown,
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

/// Request to create a balance account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBalance {
    /// Currency code (ISO 4217)
    pub currency: String,
    /// Balance type (STANDARD or SAVINGS)
    #[serde(rename = "type")]
    pub balance_type: BalanceType,
    /// Name (required for SAVINGS type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateBalance {
    /// Create a standard balance.
    pub fn standard(currency: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            balance_type: BalanceType::Standard,
            name: None,
        }
    }

    /// Create a savings balance (jar).
    pub fn savings(currency: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            balance_type: BalanceType::Savings,
            name: Some(name.into()),
        }
    }
}

/// Request for balance movement (conversion or jar transfer).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceMovementRequest {
    /// Quote ID (required for cross-currency conversions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<Uuid>,
    /// Source balance ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_balance_id: Option<i64>,
    /// Target balance ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_balance_id: Option<i64>,
    /// Amount to move (for same-currency jar transfers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
}

impl BalanceMovementRequest {
    /// Create a conversion request using a quote.
    pub fn conversion(quote_id: Uuid) -> Self {
        Self {
            quote_id: Some(quote_id),
            source_balance_id: None,
            target_balance_id: None,
            amount: None,
        }
    }

    /// Create a jar transfer request.
    pub fn jar_transfer(
        source_balance_id: i64,
        target_balance_id: i64,
        amount: Money,
    ) -> Self {
        Self {
            quote_id: None,
            source_balance_id: Some(source_balance_id),
            target_balance_id: Some(target_balance_id),
            amount: Some(amount),
        }
    }

    /// Create a cross-currency jar transfer request.
    pub fn jar_transfer_with_quote(
        source_balance_id: i64,
        target_balance_id: i64,
        quote_id: Uuid,
    ) -> Self {
        Self {
            quote_id: Some(quote_id),
            source_balance_id: Some(source_balance_id),
            target_balance_id: Some(target_balance_id),
            amount: None,
        }
    }
}

/// Balance after a step in a movement.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceAfterStep {
    /// Balance ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Value after the step
    pub value: Decimal,
    /// Currency code
    pub currency: String,
}

/// Step in a balance movement.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceMovementStep {
    /// Step ID
    pub id: i64,
    /// Step type (e.g., "CONVERSION")
    #[serde(rename = "type")]
    pub step_type: String,
    /// Creation time
    pub creation_time: DateTime<Utc>,
    /// Balances after this step
    pub balances_after: Vec<BalanceAfterStep>,
    /// Source amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_amount: Option<Money>,
    /// Target amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_amount: Option<Money>,
    /// Fee for this step
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<Money>,
    /// Exchange rate used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<Decimal>,
}

/// Response from a balance movement operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceMovement {
    /// Movement ID
    pub id: i64,
    /// Movement type (e.g., "CONVERSION")
    #[serde(rename = "type")]
    pub movement_type: String,
    /// Movement state
    pub state: String,
    /// Balances after the movement
    pub balances_after: Vec<BalanceAfterStep>,
    /// Creation time
    pub creation_time: DateTime<Utc>,
    /// Individual steps in the movement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<BalanceMovementStep>>,
    /// Total source amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_amount: Option<Money>,
    /// Total target amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_amount: Option<Money>,
    /// Exchange rate used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<Decimal>,
    /// Fee amounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amounts: Option<Vec<Money>>,
}

/// Balance capacity (deposit limits).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceCapacity {
    /// Whether there is a regulatory limit
    pub has_limit: bool,
    /// Maximum amount that can be deposited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_limit: Option<Money>,
}

/// Overdraft information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Overdraft {
    /// Available overdraft amount
    pub available: Money,
    /// Maximum overdraft limit
    pub limit: Money,
    /// Currently used overdraft
    #[serde(rename = "used")]
    pub usage: Money,
    /// Available overdraft by currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_by_currency: Option<Vec<Money>>,
}

/// Total funds information for a profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalFunds {
    /// Total worth across all balances
    pub total_worth: Money,
    /// Total available (including overdraft)
    pub total_available: Money,
    /// Overdraft information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overdraft: Option<Overdraft>,
}

/// Request to set excess money account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetExcessMoneyAccount {
    /// Recipient ID for excess money transfers
    pub recipient_id: i64,
}

/// Response from setting excess money account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcessMoneyAccount {
    /// Profile ID
    pub user_profile_id: i64,
    /// Recipient ID for excess money transfers
    pub recipient_id: i64,
}
