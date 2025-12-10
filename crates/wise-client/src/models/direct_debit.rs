//! Direct debit account models.

use serde::{Deserialize, Serialize};

/// Direct debit payment type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DirectDebitType {
    /// US direct debit.
    Ach,
    /// CA direct debit.
    Eft,
}

/// Bank account type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BankAccountType {
    Checking,
    Savings,
}

/// Direct debit account details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectDebitDetails {
    /// ABA routing number (or institution + transit for CAD).
    pub routing_number: String,
    /// Bank account number.
    pub account_number: String,
    /// Account type.
    pub account_type: BankAccountType,
}

/// Direct debit account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectDebitAccount {
    /// Account ID.
    pub id: i64,
    /// Currency code.
    pub currency: String,
    /// Payment type (ACH or EFT).
    #[serde(rename = "type")]
    pub debit_type: DirectDebitType,
    /// Account details.
    pub details: DirectDebitDetails,
}

/// Request to create a direct debit account.
#[derive(Debug, Clone, Serialize)]
pub struct CreateDirectDebitAccountRequest {
    /// Currency code (USD for ACH, CAD for EFT).
    pub currency: String,
    /// Payment type.
    #[serde(rename = "type")]
    pub debit_type: DirectDebitType,
    /// Account details.
    pub details: DirectDebitDetails,
}

impl CreateDirectDebitAccountRequest {
    /// Create a USD ACH direct debit account.
    pub fn ach(
        routing_number: impl Into<String>,
        account_number: impl Into<String>,
        account_type: BankAccountType,
    ) -> Self {
        Self {
            currency: "USD".to_string(),
            debit_type: DirectDebitType::Ach,
            details: DirectDebitDetails {
                routing_number: routing_number.into(),
                account_number: account_number.into(),
                account_type,
            },
        }
    }

    /// Create a CAD EFT direct debit account.
    pub fn eft(
        routing_number: impl Into<String>,
        account_number: impl Into<String>,
        account_type: BankAccountType,
    ) -> Self {
        Self {
            currency: "CAD".to_string(),
            debit_type: DirectDebitType::Eft,
            details: DirectDebitDetails {
                routing_number: routing_number.into(),
                account_number: account_number.into(),
                account_type,
            },
        }
    }
}
