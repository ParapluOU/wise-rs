//! Address models.

use serde::{Deserialize, Serialize};

/// Occupation information for address registration.
///
/// Required for CA, IN, JP, ID, IL, MX and US/NM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressOccupation {
    /// Occupation code - any value permitted.
    pub code: String,
    /// Occupation format - always "FREE_FORM".
    #[serde(default = "default_free_form")]
    pub format: String,
}

fn default_free_form() -> String {
    "FREE_FORM".to_string()
}

impl AddressOccupation {
    /// Create a new occupation.
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            format: "FREE_FORM".to_string(),
        }
    }
}

/// Address details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressDetails {
    /// Country code (ISO 3166-2).
    pub country: String,
    /// Address line: street, house, apartment.
    pub first_line: String,
    /// Postal/zip code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    /// City name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State code (required for US, CA, BR, AU).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Occupations (required for CA, IN, JP, ID, IL, MX and US/NM).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupations: Option<Vec<AddressOccupation>>,
}

/// A registered address record.
///
/// Represents an address that has been registered in the Wise system,
/// including its ID and details. This is the response from the Address API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredAddress {
    /// Address ID.
    pub id: i64,
    /// User profile ID.
    pub profile: i64,
    /// Address details.
    pub details: AddressDetails,
}

/// Request to create or update an address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAddressRequest {
    /// User profile ID.
    pub profile: i64,
    /// Address details.
    pub details: AddressDetails,
}

/// Field requirement information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressFieldRequirement {
    /// Field key name.
    pub key: String,
    /// Display type (text, select, etc.).
    #[serde(rename = "type")]
    pub field_type: String,
    /// Whether to refresh requirements when this field changes.
    #[serde(default)]
    pub refresh_requirements_on_change: bool,
    /// Whether the field is required.
    #[serde(default)]
    pub required: bool,
    /// Display format pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_format: Option<String>,
    /// Example value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    /// Minimum length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i32>,
    /// Maximum length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,
    /// Validation regexp pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_regexp: Option<String>,
    /// Allowed values for select fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_allowed: Option<Vec<AllowedValue>>,
}

/// Allowed value for select fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowedValue {
    /// Value key.
    pub key: String,
    /// Display name.
    pub name: String,
}

/// Field group in address requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressFieldGroup {
    /// Field name/label.
    pub name: String,
    /// Group of field requirements.
    pub group: Vec<AddressFieldRequirement>,
}

/// Address requirements response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressRequirements {
    /// Type (always "address").
    #[serde(rename = "type")]
    pub requirements_type: String,
    /// Field requirements.
    pub fields: Vec<AddressFieldGroup>,
}
