//! Recipient account models.

use serde::{Deserialize, Serialize};

/// Recipient account (where money is sent to).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipient {
    /// Account ID
    pub id: i64,
    /// Business profile ID (if applicable)
    pub business: Option<i64>,
    /// User profile ID
    pub profile: i64,
    /// Account holder name
    pub account_holder_name: String,
    /// Currency code
    pub currency: String,
    /// Country code
    pub country: Option<String>,
    /// Account type (e.g., "iban", "sort_code", "aba")
    #[serde(rename = "type")]
    pub account_type: String,
    /// Whether this is the owner's own account
    pub owned_by_customer: Option<bool>,
    /// Account details (varies by type)
    pub details: RecipientDetails,
    /// Whether the account is active
    pub active: Option<bool>,
}

/// Recipient account details.
///
/// Fields vary depending on account type (IBAN, sort code, ABA, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipientDetails {
    /// IBAN (for European accounts)
    #[serde(rename = "IBAN")]
    pub iban: Option<String>,
    /// BIC/SWIFT code
    pub bic: Option<String>,
    /// Account number
    pub account_number: Option<String>,
    /// Sort code (UK)
    pub sort_code: Option<String>,
    /// ABA routing number (US)
    pub abartn: Option<String>,
    /// Account type (checking/savings)
    pub account_type: Option<String>,
    /// Legal type (PRIVATE or BUSINESS)
    pub legal_type: Option<String>,
    /// Address
    pub address: Option<RecipientAddress>,
    /// Email (for email recipients)
    pub email: Option<String>,
}

/// Recipient address.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipientAddress {
    /// First line of address
    pub first_line: Option<String>,
    /// City
    pub city: Option<String>,
    /// State or region
    pub state: Option<String>,
    /// Postal code
    pub post_code: Option<String>,
    /// Country code
    pub country: Option<String>,
}

/// Request to create a recipient account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecipient {
    /// Profile ID
    pub profile: i64,
    /// Account holder name
    pub account_holder_name: String,
    /// Currency code
    pub currency: String,
    /// Account type
    #[serde(rename = "type")]
    pub account_type: String,
    /// Account details
    pub details: serde_json::Value,
    /// Whether this is a refund account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<bool>,
}
