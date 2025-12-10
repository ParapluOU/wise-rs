//! Transfer models.

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::datetime;

/// Money transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    /// Transfer ID
    pub id: i64,
    /// User ID who created the transfer
    pub user: i64,
    /// Target recipient account ID
    pub target_account: i64,
    /// Source balance account ID
    pub source_account: Option<i64>,
    /// Associated quote ID
    pub quote: Option<i64>,
    /// Quote UUID
    pub quote_uuid: Option<String>,
    /// Transfer status
    pub status: TransferStatus,
    /// Payment reference
    pub reference: Option<String>,
    /// Exchange rate applied
    pub rate: Decimal,
    /// Creation timestamp
    #[serde(deserialize_with = "datetime::deserialize")]
    pub created: NaiveDateTime,
    /// Business profile ID if applicable
    pub business: Option<i64>,
    /// Transfer details
    pub details: Option<TransferDetails>,
    /// Whether the transfer has active issues
    pub has_active_issues: Option<bool>,
    /// Source currency code
    pub source_currency: String,
    /// Source amount
    pub source_value: Decimal,
    /// Target currency code
    pub target_currency: String,
    /// Target amount
    pub target_value: Decimal,
    /// Customer transaction ID for idempotency
    pub customer_transaction_id: Option<String>,
}

/// Transfer status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferStatus {
    /// Waiting for incoming payment
    IncomingPaymentWaiting,
    /// Incoming payment initiated
    IncomingPaymentInitiated,
    /// Processing the transfer
    Processing,
    /// Funds have been converted
    FundsConverted,
    /// Outgoing payment sent
    OutgoingPaymentSent,
    /// Transfer cancelled
    Cancelled,
    /// Funds refunded
    FundsRefunded,
    /// Payment bounced back
    Bounced,
    /// Unknown status
    #[serde(other)]
    Unknown,
}

impl TransferStatus {
    /// Returns true if the transfer is in a final state.
    pub fn is_final(&self) -> bool {
        matches!(
            self,
            TransferStatus::OutgoingPaymentSent
                | TransferStatus::Cancelled
                | TransferStatus::FundsRefunded
                | TransferStatus::Bounced
        )
    }

    /// Returns true if the transfer is still in progress.
    pub fn is_pending(&self) -> bool {
        matches!(
            self,
            TransferStatus::IncomingPaymentWaiting
                | TransferStatus::IncomingPaymentInitiated
                | TransferStatus::Processing
                | TransferStatus::FundsConverted
        )
    }
}

/// Transfer details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferDetails {
    /// Payment reference visible to recipient
    pub reference: Option<String>,
    /// Transfer purpose
    pub transfer_purpose: Option<String>,
    /// Source of funds
    pub source_of_funds: Option<String>,
}

/// Request to create a transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransfer {
    /// Target recipient account ID
    pub target_account: i64,
    /// Quote UUID
    pub quote_uuid: String,
    /// Customer transaction ID (UUID) for idempotency
    pub customer_transaction_id: String,
    /// Transfer details
    pub details: CreateTransferDetails,
}

/// Details for creating a transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransferDetails {
    /// Payment reference
    pub reference: String,
    /// Transfer purpose (required for some corridors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_purpose: Option<String>,
    /// Source of funds (required for some corridors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_of_funds: Option<String>,
}

/// Request to fund a transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundTransfer {
    /// Payment type (e.g., "BALANCE")
    #[serde(rename = "type")]
    pub payment_type: String,
}

impl FundTransfer {
    /// Create a request to fund from balance.
    pub fn from_balance() -> Self {
        Self {
            payment_type: "BALANCE".to_string(),
        }
    }
}
