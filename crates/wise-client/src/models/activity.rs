//! Activity models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Activity resource type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityResourceType {
    AccrualCharge,
    AcquiringPayment,
    AssetsWithdrawal,
    BalanceCashback,
    BalanceInterest,
    BalanceTransaction,
    BankDetailsOrder,
    BatchTransfer,
    CardCashback,
    CardOrder,
    CardTransaction,
    DirectDebitInstruction,
    DirectDebitTransaction,
    FeeRefund,
    IncidentRefund,
    IncorporationOrder,
    OperationalTransaction,
    PaymentRequest,
    Reward,
    RewardsRedemption,
    SendOrder,
    SendOrderExecution,
    Transfer,
    #[serde(other)]
    Unknown,
}

/// Activity type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityType {
    AcquiringPayment,
    AutoConversion,
    BalanceAdjustment,
    BalanceAssetFee,
    BalanceCashback,
    BalanceDeposit,
    BalanceHoldFee,
    BalanceInterest,
    BankDetailsOrder,
    BatchTransfer,
    CardCashback,
    CardCheck,
    CardOrder,
    CardPayment,
    CashWithdrawal,
    ClaimableSendOrder,
    DirectDebitTransaction,
    ExcessRefund,
    FeeRefund,
    IncorporationOrder,
    Interbalance,
    PaymentRequest,
    PrefundingTransfer,
    Reward,
    ScheduledSendOrder,
    Transfer,
    #[serde(other)]
    Unknown,
}

/// Activity status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityStatus {
    /// Requires end user attention.
    RequiresAttention,
    /// Activity has yet to be completed.
    InProgress,
    /// Activity is scheduled for the future.
    Upcoming,
    /// Activity is at its end state.
    Completed,
    /// Activity is cancelled.
    Cancelled,
    #[serde(other)]
    Unknown,
}

/// Activity resource reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityResource {
    /// Resource type.
    #[serde(rename = "type")]
    pub resource_type: ActivityResourceType,
    /// Resource ID.
    pub id: String,
}

/// Activity object representing a snapshot of a performed action.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    /// Activity ID.
    pub id: String,
    /// Type of activity.
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    /// Activity resource reference.
    pub resource: ActivityResource,
    /// Title of the activity (may contain custom tags like <strong>, <positive>, <negative>).
    pub title: String,
    /// Short description summarizing the activity.
    #[serde(default)]
    pub description: String,
    /// Primary amount of the transaction (e.g., "100 USD").
    #[serde(default)]
    pub primary_amount: Option<String>,
    /// Secondary amount of the transaction (e.g., "80 GBP").
    #[serde(default)]
    pub secondary_amount: Option<String>,
    /// Status of the activity.
    pub status: ActivityStatus,
    /// Timestamp when the activity was created.
    pub created_on: DateTime<Utc>,
    /// Timestamp when the activity was last modified.
    pub updated_on: DateTime<Utc>,
}

/// Response for listing activities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivitiesResponse {
    /// Cursor for pagination. Use as `nextCursor` to get the next page.
    pub cursor: Option<String>,
    /// List of activities.
    pub activities: Vec<Activity>,
}

/// Parameters for listing activities.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListActivitiesParams {
    /// Filter by resource type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monetary_resource_type: Option<ActivityResourceType>,
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ActivityStatus>,
    /// Filter activities after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<DateTime<Utc>>,
    /// Filter activities until this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<Utc>>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Page size (1-100, default 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
}
