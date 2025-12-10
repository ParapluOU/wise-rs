//! Bank account details models.

use serde::{Deserialize, Serialize};

/// Bank account details status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BankDetailsStatus {
    /// Account details do not exist but may be created.
    Available,
    /// Account details are ready to use.
    Active,
}

/// Receive option type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReceiveOptionType {
    /// Local bank details to receive money in account currency.
    Local,
    /// Swift bank details to receive money internationally.
    International,
}

/// Receive option detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveOptionDetail {
    /// Account detail type (e.g., ACCOUNT_HOLDER, SWIFT_CODE, IBAN).
    #[serde(rename = "type")]
    pub detail_type: String,
    /// Label to display in UI.
    pub title: String,
    /// Value to display in UI.
    pub body: String,
    /// Content for tooltip/popup hint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the field should be hidden in UI.
    #[serde(default)]
    pub hidden: bool,
}

/// Receive option.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveOption {
    /// Option type (Local or International).
    #[serde(rename = "type")]
    pub option_type: ReceiveOptionType,
    /// Account details for this receive option.
    pub details: Vec<ReceiveOptionDetail>,
}

/// Bank feature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankFeature {
    /// Feature key (e.g., LOCAL_RECEIVE, SWIFT, DIRECT_DEBITS).
    pub key: String,
    /// Feature title.
    pub title: String,
    /// Whether the feature is supported.
    pub supported: bool,
}

/// Currency information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankDetailsCurrency {
    /// Currency code (ISO 4217).
    pub code: String,
    /// Currency name.
    pub name: String,
}

/// Bank account details object.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankAccountDetails {
    /// Bank account details ID.
    pub id: Option<i64>,
    /// Currency information.
    pub currency: BankDetailsCurrency,
    /// Account title.
    pub title: String,
    /// Account subtitle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    /// Status.
    pub status: BankDetailsStatus,
    /// Whether details are deprecated (new ones issued).
    #[serde(default)]
    pub deprecated: bool,
    /// Available receive options.
    #[serde(default)]
    pub receive_options: Vec<ReceiveOption>,
    /// Features enabled on the account.
    #[serde(default)]
    pub bank_features: Vec<BankFeature>,
}

/// Bank account details order requirement type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderRequirementType {
    /// User needs to be fully verified.
    Verification,
    /// A fee must be paid.
    TopUp,
}

/// Bank account details order requirement status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderRequirementStatus {
    /// Pending action from user.
    PendingUser,
    /// Pending action from Wise.
    PendingTw,
    /// Requirement completed.
    Done,
}

/// Order requirement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequirement {
    /// Requirement type.
    #[serde(rename = "type")]
    pub requirement_type: OrderRequirementType,
    /// Requirement status.
    pub status: OrderRequirementStatus,
}

/// Bank account details order status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    PendingUser,
    PendingTw,
    RequirementsFulfilled,
    Done,
}

/// Bank account details order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankDetailsOrder {
    /// Order status.
    pub status: OrderStatus,
    /// Currency code.
    pub currency: String,
    /// Requirements for the order.
    pub requirements: Vec<OrderRequirement>,
}

/// Request to create a bank account details order.
#[derive(Debug, Clone, Serialize)]
pub struct CreateBankDetailsOrderRequest {
    /// Currency code (ISO 4217).
    pub currency: String,
}

/// Request to create multiple bank account details.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMultipleBankDetailsRequest {
    /// ID of the currency balance to create account details on.
    pub target_account_id: i64,
}

/// Local bank details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalBankDetails {
    pub bank_name: Option<String>,
    pub bank_address: Option<String>,
    pub sort_code: Option<String>,
    pub account_number: Option<String>,
    #[serde(rename = "type")]
    pub details_type: Option<String>,
}

/// International bank details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternationalBankDetails {
    pub bank_name: Option<String>,
    pub bank_address: Option<String>,
    pub swift_code: Option<String>,
    pub iban: Option<String>,
    #[serde(rename = "type")]
    pub details_type: Option<String>,
}

/// Response for creating multiple bank details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleBankDetailsResponse {
    pub id: String,
    pub currency: String,
    pub active: bool,
    pub local_details: Option<LocalBankDetails>,
    pub international_details: Option<InternationalBankDetails>,
}

/// Reason for payment return.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentReturnReason {
    IncorrectAccountNumber,
    ClosedAccount,
    BlockedAccount,
    CancellationRequest,
    Regulatory,
    CustomerRequest,
}

/// Request to return a payment.
#[derive(Debug, Clone, Serialize)]
pub struct CreatePaymentReturnRequest {
    /// Reason for the return (required for Swift payments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<PaymentReturnReason>,
}

/// Response for creating a payment return.
#[derive(Debug, Clone, Deserialize)]
pub struct PaymentReturnResponse {
    /// ID of the return created.
    pub id: String,
}
