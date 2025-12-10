//! Bulk settlement models.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Transfer in settlement journal.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlementTransfer {
    /// Transfer ID.
    pub id: i64,
    /// Date the transfer was created (ISO8601).
    pub date: String,
    /// Transfer source amount with fee.
    pub source_amount: Decimal,
    /// Transfer source currency.
    pub source_currency: String,
    /// Customer name.
    pub customer_name: String,
    /// Partner's transaction identifier.
    pub partner_reference: String,
    /// Additional data about the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Exchange rate (required for cross-currency settlement).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<Decimal>,
}

/// Refunded transfer in settlement journal.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefundedTransfer {
    /// Transfer ID.
    pub id: i64,
    /// Partner's transaction identifier.
    pub partner_reference: String,
    /// Exchange rate used for original settlement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<Decimal>,
}

/// Settlement journal request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlementJournal {
    /// Type (always "TRUSTED_BULK_SETTLEMENT").
    #[serde(rename = "type")]
    pub settlement_type: String,
    /// Reference for settlement payment (max 10 chars, starts with TPFB).
    pub settlement_reference: String,
    /// Date of settlement (ISO8601).
    pub settlement_date: String,
    /// Settlement currency (for cross-currency settlement).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_currency: Option<String>,
    /// Transfers to settle.
    pub transfers: Vec<SettlementTransfer>,
    /// Refunded transfers (for net settlement).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_transfers: Option<Vec<RefundedTransfer>>,
    /// Amount to deduct from settlement (must be 0 or negative).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transfer: Option<Decimal>,
}

impl SettlementJournal {
    /// Create a new settlement journal.
    pub fn new(
        settlement_reference: impl Into<String>,
        settlement_date: impl Into<String>,
        transfers: Vec<SettlementTransfer>,
    ) -> Self {
        Self {
            settlement_type: "TRUSTED_BULK_SETTLEMENT".to_string(),
            settlement_reference: settlement_reference.into(),
            settlement_date: settlement_date.into(),
            settlement_currency: None,
            transfers,
            refunded_transfers: None,
            balance_transfer: None,
        }
    }

    /// Set cross-currency settlement.
    pub fn with_settlement_currency(mut self, currency: impl Into<String>) -> Self {
        self.settlement_currency = Some(currency.into());
        self
    }

    /// Add refunded transfers.
    pub fn with_refunded_transfers(mut self, refunds: Vec<RefundedTransfer>) -> Self {
        self.refunded_transfers = Some(refunds);
        self
    }

    /// Set balance transfer amount.
    pub fn with_balance_transfer(mut self, amount: Decimal) -> Self {
        self.balance_transfer = Some(amount);
        self
    }
}
