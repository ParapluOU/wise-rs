//! Balance statement models.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Statement type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StatementType {
    /// Single statement line per transaction.
    Compact,
    /// Accounting statements with fees on separate lines.
    Flat,
}

/// Account holder type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountHolderType {
    Personal,
    Business,
}

/// Account holder address.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountHolderAddress {
    pub address_first_line: Option<String>,
    pub city: Option<String>,
    pub post_code: Option<String>,
    pub state_code: Option<String>,
    pub country_name: Option<String>,
}

/// Account holder information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountHolder {
    #[serde(rename = "type")]
    pub holder_type: AccountHolderType,
    pub address: AccountHolderAddress,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

/// Issuer information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issuer {
    pub name: String,
    pub first_line: Option<String>,
    pub city: Option<String>,
    pub post_code: Option<String>,
    pub state_code: Option<String>,
    pub country: Option<String>,
}

/// Statement money amount.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementMoney {
    pub value: Decimal,
    pub currency: String,
}

/// Transaction type in statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StatementTransactionType {
    Debit,
    Credit,
}

/// Transaction details type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionDetailsType {
    Card,
    Conversion,
    Deposit,
    Transfer,
    MoneyAdded,
    IncomingCrossBalance,
    OutgoingCrossBalance,
    DirectDebit,
    BalanceInterest,
    BalanceAdjustment,
    AccrualCharge,
    InvestmentTradeOrder,
    AcquiringPayment,
    CardCashback,
    CardOrderCheckout,
    Unknown,
    #[serde(other)]
    Other,
}

/// Merchant information in transaction details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatementMerchant {
    pub name: Option<String>,
    pub first_line: Option<String>,
    pub post_code: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub category: Option<String>,
}

/// Transaction details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDetails {
    #[serde(rename = "type")]
    pub details_type: TransactionDetailsType,
    pub description: Option<String>,
    pub amount: Option<StatementMoney>,
    pub sender_name: Option<String>,
    pub sender_account: Option<String>,
    pub payment_reference: Option<String>,
    pub category: Option<String>,
    pub merchant: Option<StatementMerchant>,
    pub source_amount: Option<StatementMoney>,
    pub target_amount: Option<StatementMoney>,
    pub fee: Option<StatementMoney>,
    pub rate: Option<Decimal>,
}

/// Exchange details in transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeDetails {
    pub to_amount: Option<StatementMoney>,
    pub from_amount: Option<StatementMoney>,
    pub for_amount: Option<StatementMoney>,
    pub rate: Option<Decimal>,
}

/// Statement transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatementTransaction {
    #[serde(rename = "type")]
    pub transaction_type: StatementTransactionType,
    pub date: DateTime<Utc>,
    pub amount: StatementMoney,
    pub total_fees: StatementMoney,
    pub details: TransactionDetails,
    pub exchange_details: Option<ExchangeDetails>,
    pub running_balance: StatementMoney,
    pub reference_number: String,
}

/// Statement query parameters (echoed in response).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatementQuery {
    pub interval_start: DateTime<Utc>,
    pub interval_end: DateTime<Utc>,
    pub currency: String,
    pub account_id: i64,
}

/// Balance statement response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceStatement {
    pub account_holder: AccountHolder,
    pub issuer: Issuer,
    pub bank_details: Option<serde_json::Value>,
    pub transactions: Vec<StatementTransaction>,
    pub end_of_statement_balance: StatementMoney,
    pub query: StatementQuery,
}
