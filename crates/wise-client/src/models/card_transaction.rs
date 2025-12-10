//! Card transaction models.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Card transaction state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardTransactionState {
    /// Authorized but not captured.
    InProgress,
    /// Captured and/or settled.
    Completed,
    /// Transaction declined.
    Declined,
    /// Transaction cancelled.
    Cancelled,
    /// Unknown state.
    Unknown,
}

/// Card transaction type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardTransactionType {
    /// Receiving money on card.
    AccountCredit,
    /// Sending money to another card/e-wallet.
    AccountFunding,
    /// Cash disbursement.
    CashAdvance,
    /// ATM withdrawal.
    CashWithdrawal,
    /// Reserved for future use.
    Chargeback,
    /// Visa OCT / Mastercard MoneySend.
    CreditTransaction,
    /// Online purchase.
    EcomPurchase,
    /// POS terminal purchase.
    PosPurchase,
    /// Partial or full refund.
    Refund,
}

/// Transaction amount.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionAmount {
    pub amount: Decimal,
    pub currency: String,
}

/// Transaction fee.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionFee {
    pub amount: Decimal,
    pub currency: String,
    pub fee_type: String,
}

/// Merchant location.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerchantLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Merchant category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerchantCategory {
    pub name: String,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Merchant information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Merchant {
    pub name: String,
    pub location: MerchantLocation,
    pub category: MerchantCategory,
}

/// Debit from balance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDebit {
    pub balance_id: i64,
    pub debited_amount: TransactionAmount,
    pub for_amount: TransactionAmount,
    pub rate: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<TransactionAmount>,
}

/// Credit to balance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionCredit {
    pub balance_id: i64,
    pub credited_amount: TransactionAmount,
}

/// Card transaction decline reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeclineReason {
    AccountInactive,
    AccountSuspended,
    AtmPinChangeNotAllowed,
    BlockedCountry,
    BlockedSubscription,
    CardExpired,
    CardFrozen,
    CardInactive,
    CardBlocked,
    CardPartnerSuspended,
    ChipPinEntryTriesExceeded,
    ConnectionIssue,
    ContactlessPinEntryRequired,
    CreditTransactionsNotSupported,
    CumulativeLimitExceeded,
    DeclinedAdvice,
    IncorrectCvv,
    IncorrectExpiryDate,
    IncorrectPin,
    InsufficientFunds,
    Invalid3dsUcaf,
    IncorrectArqc,
    IncorrectIcvv,
    InvalidMerchant,
    InvalidTransaction,
    MandateDccNonSupportedForCardCountry,
    MandateLocalCashWithdrawalNotAllowed,
    NonSupportedCurrency,
    NonSupportedMccForCountry,
    PaymentMethodDailyLimitExceeded,
    PaymentMethodLifetimeLimitExceeded,
    PaymentMethodMonthlyLimitExceeded,
    PaymentMethodNotAllowed,
    PaymentMethodTransactionLimitExceeded,
    PinEntryTriesExceeded,
    PreActivatedCardPinEntryRequired,
    ProcessingError,
    RestrictedMode,
    ReversalNotMatchingAuthCurrency,
    ScaSoftDecline,
    SchemeBlockedTransaction,
    SecurityCvmFailure,
    SecurityMagstripeSecureElementsIncorrectOrMissing,
    SecurityPinEntryRequired,
    SuspectedFraud,
    SuspectedFraudAml,
    SuspectedFraudCompliance,
    SuspectedFraudCoreFraud,
    SuspectedFraudSanctions,
    SuspectedFraudSoftDecline,
    TransactionTypeNotSupported,
    UnexpectedError,
    #[serde(other)]
    Unknown,
}

/// Card transaction resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardTransaction {
    /// Transaction ID.
    pub id: String,
    /// Card token.
    pub card_token: String,
    /// Transaction type.
    #[serde(rename = "type")]
    pub transaction_type: CardTransactionType,
    /// Decline reason (if declined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_reason: Option<DeclineReason>,
    /// When transaction occurred.
    pub created_date: DateTime<Utc>,
    /// Current state.
    pub state: CardTransactionState,
    /// Last 4 digits of card.
    pub card_last_digits: String,
    /// Transaction amount.
    pub transaction_amount: TransactionAmount,
    /// Fees.
    #[serde(default)]
    pub fees: Vec<TransactionFee>,
    /// Amount including fees.
    pub transaction_amount_with_fees: TransactionAmount,
    /// Merchant information.
    pub merchant: Merchant,
    /// Authorisation method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorisation_method: Option<String>,
    /// Associated balance transaction ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction_id: Option<i64>,
    /// Debits from balances.
    #[serde(default)]
    pub debits: Vec<TransactionDebit>,
    /// Credit to balance (for refunds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<TransactionCredit>,
}
