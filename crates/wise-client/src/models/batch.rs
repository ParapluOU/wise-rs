//! Batch group models.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Batch group status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BatchGroupStatus {
    /// New batch group, can have transfers added.
    New,
    /// Closed to changes, transfers can be funded.
    Completed,
    /// Cancellation requested.
    MarkedForCancellation,
    /// Transfers being cancelled.
    ProcessingCancel,
    /// All transfers cancelled.
    Cancelled,
}

/// Pay in details address.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayInAddress {
    pub name: Option<String>,
    pub first_line: Option<String>,
    pub post_code: Option<String>,
    pub city: Option<String>,
    pub state_code: Option<String>,
    pub country: Option<String>,
}

/// Pay in details for funding a batch group.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayInDetails {
    /// Payment method type (e.g., "bank_transfer").
    #[serde(rename = "type")]
    pub payment_type: String,
    /// Reference to use when funding.
    pub reference: String,
    /// Total pay in amount.
    pub amount: Decimal,
    /// Currency code.
    pub currency: String,
    /// Bank account holder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Bank branch name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// Bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Bank account type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Bank identifier or routing number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    /// Bank address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_address: Option<PayInAddress>,
    /// Wise's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_wise_address: Option<PayInAddress>,
    /// IBAN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    /// BBAN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bban: Option<String>,
    /// Financial institution number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_number: Option<String>,
    /// Branch transit number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_number: Option<String>,
    /// Beneficiary bank BIC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_bank_bic: Option<String>,
    /// Intermediary bank BIC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediary_bank_bic: Option<String>,
    /// FPS identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps_identifier: Option<String>,
    /// Clearing number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearing_number: Option<String>,
}

/// Batch group object.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchGroup {
    /// Batch group ID.
    pub id: Uuid,
    /// Version for concurrency control.
    pub version: i64,
    /// Descriptive name.
    pub name: String,
    /// Source currency code.
    pub source_currency: String,
    /// Current status.
    pub status: BatchGroupStatus,
    /// Transfer IDs in the group.
    #[serde(default)]
    pub transfer_ids: Vec<i64>,
    /// Pay in details (only when COMPLETED).
    #[serde(default)]
    pub pay_in_details: Vec<PayInDetails>,
}

/// Request to create a batch group.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBatchGroupRequest {
    /// Source currency code.
    pub source_currency: String,
    /// Descriptive name (max 100 characters).
    pub name: String,
}

/// Request to update batch group status.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateBatchGroupRequest {
    /// New status (COMPLETED or CANCELLED).
    pub status: String,
    /// Expected version for concurrency control.
    pub version: i64,
}

/// Batch payment type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BatchPaymentType {
    Balance,
    DirectDebit,
}

/// Request to fund a batch group.
#[derive(Debug, Clone, Serialize)]
pub struct FundBatchGroupRequest {
    /// Payment type (must be BALANCE).
    #[serde(rename = "type")]
    pub payment_type: BatchPaymentType,
}

/// Response for funding a batch group.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchPaymentResponse {
    pub id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    pub already_paid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_id: Option<i64>,
    pub user_id: i64,
    pub profile_id: i64,
    pub source_currency: String,
    pub status: String,
    pub group_type: String,
    #[serde(default)]
    pub transfer_ids: Vec<i64>,
}

/// Request to fund via direct debit.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FundBatchDirectDebitRequest {
    /// Payment type (must be DIRECT_DEBIT).
    #[serde(rename = "type")]
    pub payment_type: String,
    /// Direct debit account ID.
    pub account_id: i64,
    /// Optional payment reference (max 10 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// Payment initiation status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentInitiationStatus {
    New,
    Processing,
    Completed,
    Failed,
    ChargedBack,
}

/// Payment initiation response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInitiation {
    pub id: i64,
    pub batch_group_id: Uuid,
    pub reference: String,
    pub user_id: i64,
    pub profile_id: i64,
    #[serde(rename = "type")]
    pub payment_type: String,
    pub status: PaymentInitiationStatus,
    pub account_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<i64>,
    pub created_time: DateTime<Utc>,
}
