//! 3D Secure authentication models.

use serde::{Deserialize, Serialize};

/// 3DS challenge status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChallengeStatus {
    Approved,
    Rejected,
}

/// Request to inform 3DS challenge result.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeResultRequest {
    /// Transaction reference from webhook event.
    pub transaction_reference: String,
    /// Challenge status (APPROVED or REJECTED).
    pub challenge_status: ChallengeStatus,
}
