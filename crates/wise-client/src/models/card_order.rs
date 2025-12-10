//! Card order models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::card::CardProgram;

/// Card order status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardOrderStatus {
    /// Order created, waiting for requirements.
    Placed,
    /// Requirements fulfilled, card being generated.
    RequirementsFulfilled,
    /// Card details created.
    CardDetailsCreated,
    /// Physical card produced, waiting for delivery.
    Produced,
    /// Card activated and ready to use.
    Completed,
    /// Order cancelled.
    Cancelled,
    /// Physical card returned, will be blocked.
    Returned,
}

/// Delivery option.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeliveryOption {
    PostalServiceStandard,
    PostalServiceWithTracking,
    KioskCollection,
}

/// Card order address.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardOrderAddress {
    pub first_line: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_line: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_line: Option<String>,
    pub city: String,
    pub post_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    pub country: String,
}

/// Delivery details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
    pub delivery_option: DeliveryOption,
}

/// Card order resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardOrder {
    /// Card order ID.
    pub id: i64,
    /// Profile ID.
    pub profile_id: i64,
    /// Client ID.
    pub client_id: String,
    /// Card program.
    pub card_program: CardProgram,
    /// Billing/delivery address.
    pub address: CardOrderAddress,
    /// Card token (after card is created).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_token: Option<String>,
    /// Replaced card token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces_card: Option<String>,
    /// Creation time.
    pub creation_time: DateTime<Utc>,
    /// Last modification time.
    pub modification_time: DateTime<Utc>,
    /// Order status.
    pub status: CardOrderStatus,
    /// Cardholder name.
    pub card_holder_name: String,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Lifetime spending limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifetime_limit: Option<i64>,
    /// Estimated delivery time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<DateTime<Utc>>,
    /// Delivery details (physical cards only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_details: Option<DeliveryDetails>,
    /// Cardholder profile ID (for business cards).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_holder_profile_id: Option<i64>,
}

/// Response for listing card orders.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardOrdersResponse {
    pub total_count: i64,
    pub card_orders: Vec<CardOrder>,
}

/// Card programs availability response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardProgramsResponse {
    pub card_programs: Vec<CardProgram>,
}

/// Replacement reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReplacementReason {
    CardDamaged,
    CardExpiring,
}

/// Replacement details.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplacementDetails {
    /// Token of card to replace.
    pub card_token: String,
    /// Reason for replacement.
    pub reason: ReplacementReason,
}

/// Request to create a card order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCardOrderRequest {
    /// Card program name.
    pub program: String,
    /// Cardholder name.
    pub card_holder_name: String,
    /// Name to emboss on physical card (max 22 chars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embossed_name: Option<String>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Billing/delivery address.
    pub address: CardOrderAddress,
    /// Delivery option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_option: Option<DeliveryOption>,
    /// Lifetime spending limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifetime_limit: Option<i64>,
    /// Cardholder profile ID (for business cards).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_holder_profile_id: Option<i64>,
    /// Replacement details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_details: Option<ReplacementDetails>,
}

/// Card order requirement type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardOrderRequirementType {
    Pin,
    Verification,
    Address,
}

/// Card order requirement status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardOrderRequirementStatus {
    NotInitiated,
    NeedsAction,
    Pending,
    Completed,
    Failed,
}

/// Card order requirement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardOrderRequirement {
    #[serde(rename = "type")]
    pub requirement_type: CardOrderRequirementType,
    pub status: CardOrderRequirementStatus,
}

/// Response for card order requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardOrderRequirementsResponse {
    pub requirements: Vec<CardOrderRequirement>,
}

/// Request to update card order status.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateCardOrderStatusRequest {
    /// New status (CANCELLED or COMPLETED).
    pub status: String,
}

/// Request to validate an address.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateAddressRequest {
    pub first_line: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_line: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_line: Option<String>,
    pub city: String,
    pub post_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    pub country: String,
}

/// Request to set card PIN.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCardPinRequest {
    /// Key version (always 1).
    pub key_version: i32,
    /// JWE encrypted payload.
    pub encrypted_payload: String,
}

/// Response for setting card PIN.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCardPinResponse {
    pub card_order_id: String,
}
