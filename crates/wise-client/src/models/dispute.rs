//! Dispute models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Dispute status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DisputeStatus {
    Active,
    Closed,
}

/// Dispute sub-status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DisputeSubStatus {
    /// Initial status.
    Submitted,
    /// Under review or needs more info.
    InReview,
    /// Refund processed.
    Refunded,
    /// Dispute invalid.
    Rejected,
    /// Customer withdrew dispute.
    Withdrawn,
    /// Reviewed but refund not applicable.
    Confirmed,
    /// Refund being processed.
    RefundInProgress,
    /// Chargeback submitted.
    AttemptingRecovery,
    /// Chargeback unsuccessful.
    RecoveryUnsuccessful,
}

/// Dispute resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dispute {
    /// Dispute ID.
    pub id: String,
    /// Card transaction ID.
    pub transaction_id: i64,
    /// Profile ID.
    pub profile_id: i64,
    /// Dispute reason code.
    pub reason: String,
    /// Overall status.
    pub status: DisputeStatus,
    /// Detailed status.
    pub sub_status: DisputeSubStatus,
    /// Explanation for sub-status.
    pub status_message: String,
    /// When the dispute was created.
    pub created_at: DateTime<Utc>,
    /// Who created the dispute (user ID).
    pub created_by: String,
    /// When the dispute was last updated.
    pub last_updated_at: DateTime<Utc>,
    /// Whether the dispute can be withdrawn.
    pub can_withdraw: bool,
}

/// Response for listing disputes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisputesResponse {
    pub total_count: i64,
    pub disputes: Vec<Dispute>,
}

/// Dispute reason sub-option.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisputeReasonSubOption {
    /// Reason code to use for submitting dispute.
    pub code: String,
    /// Description.
    pub description: String,
    /// Whether this is fraud-related.
    #[serde(default)]
    pub is_fraud: bool,
    /// Whether multiple transactions can be disputed.
    #[serde(default)]
    pub supports_multiple_transactions: bool,
    /// Tooltip text for UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
}

/// Dispute reason.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisputeReason {
    /// Reason code.
    pub code: String,
    /// Description.
    pub description: String,
    /// Whether this is fraud-related.
    #[serde(default)]
    pub is_fraud: bool,
    /// Whether multiple transactions can be disputed.
    #[serde(default)]
    pub supports_multiple_transactions: bool,
    /// Tooltip text for UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
    /// Sub-options (use these as dispute reason if present).
    #[serde(default)]
    pub sub_options: Vec<DisputeReasonSubOption>,
}

/// Request to initiate dispute flow.
#[derive(Debug, Clone, Serialize)]
pub struct InitiateDisputeFlowRequest {
    /// Email for dispute communications.
    pub email: String,
}

/// Request to withdraw a dispute.
#[derive(Debug, Clone, Serialize)]
pub struct WithdrawDisputeRequest {
    /// Status to set (must be "CLOSED").
    pub status: String,
}

impl WithdrawDisputeRequest {
    /// Create a withdraw request.
    pub fn new() -> Self {
        Self {
            status: "CLOSED".to_string(),
        }
    }
}

impl Default for WithdrawDisputeRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Response for uploading dispute file.
#[derive(Debug, Clone, Deserialize)]
pub struct DisputeFileResponse {
    /// Map of filename to file ID.
    #[serde(flatten)]
    pub files: std::collections::HashMap<String, String>,
}
