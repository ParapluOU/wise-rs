//! Card models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Card status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardStatus {
    /// Card is active and can be used.
    Active,
    /// Card is inactive, transactions will be declined.
    Inactive,
    /// Card is permanently blocked.
    Blocked,
    /// Card is temporarily frozen.
    Frozen,
    /// Card is suspended by Wise (internal use).
    PartnerSuspended,
    /// Card has expired.
    Expired,
    /// Cardholder data has been purged.
    Purged,
}

/// Card status wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardStatusWrapper {
    pub value: CardStatus,
}

/// Card type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardType {
    Physical,
    VirtualNonUpgradeable,
    #[serde(other)]
    Unknown,
}

/// Card scheme.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardScheme {
    Visa,
    Mastercard,
    #[serde(other)]
    Unknown,
}

/// Card program.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardProgram {
    /// Card program name.
    pub name: String,
    /// Card scheme (VISA, MASTERCARD).
    pub scheme: CardScheme,
    /// Default currency.
    pub default_currency: String,
    /// Card type.
    pub card_type: CardType,
}

/// Card resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    /// Card token (unique identifier).
    pub token: String,
    /// Profile ID.
    pub profile_id: i64,
    /// Client ID.
    pub client_id: String,
    /// Card status.
    pub status: CardStatusWrapper,
    /// Cardholder name.
    pub card_holder_name: String,
    /// Expiry date.
    pub expiry_date: DateTime<Utc>,
    /// Last 4 digits of card number.
    pub last_four_digits: String,
    /// Bank identification number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_identification_number: Option<String>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Card program.
    pub card_program: CardProgram,
    /// When the card was created.
    pub creation_time: DateTime<Utc>,
    /// When the card was last modified.
    pub modification_time: DateTime<Utc>,
}

/// Response for listing cards.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardsResponse {
    /// Total number of cards.
    pub total_count: i64,
    /// List of cards.
    pub cards: Vec<Card>,
}

/// Request to update card status.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateCardStatusRequest {
    /// New status (ACTIVE, FROZEN, or BLOCKED).
    pub status: String,
}

/// Permission type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PermissionType {
    /// E-commerce transactions.
    Ecom,
    /// Chip and PIN transactions.
    PosChip,
    /// ATM withdrawals.
    AtmWithdrawal,
    /// Mobile wallet transactions.
    MobileWallets,
    /// Contactless transactions.
    PosContactless,
    /// Magnetic stripe transactions.
    PosMagstripe,
}

/// Card spending permission.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpendingPermission {
    /// Permission type.
    #[serde(rename = "type")]
    pub permission_type: PermissionType,
    /// Whether permission is enabled.
    pub is_enabled: bool,
    /// Whether permission is locked.
    pub is_locked: bool,
}

/// Response for getting spending permissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingPermissionsResponse {
    pub permissions: Vec<SpendingPermission>,
}

/// Request to modify spending permissions.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSpendingPermissionRequest {
    /// Permission type.
    #[serde(rename = "type")]
    pub permission_type: PermissionType,
    /// Whether to enable the permission.
    pub is_enabled: bool,
}

/// Request for sensitive card data.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SensitiveCardDataRequest {
    /// Key version (always 1).
    pub key_version: i32,
    /// JWE encrypted payload.
    pub encrypted_payload: String,
}

/// Response with sensitive card details.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SensitiveCardDetailsResponse {
    /// Nonce from cryptographic communication.
    pub nonce: String,
    /// Card CVV2.
    pub cvv2: String,
    /// Card PAN.
    pub pan: String,
    /// Card expiry date.
    pub expiry_date: String,
    /// Cardholder name.
    pub cardholder_name: String,
}

/// Response with card PIN.
#[derive(Debug, Clone, Deserialize)]
pub struct CardPinResponse {
    /// Nonce from cryptographic communication.
    pub nonce: String,
    /// Card PIN.
    pub pin: String,
}
