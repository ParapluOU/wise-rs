//! User model.

use serde::{Deserialize, Serialize};

/// Address information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// City name
    pub city: Option<String>,
    /// 2-letter country code (e.g., "US")
    pub country_code: Option<String>,
    /// Postal/ZIP code
    pub post_code: Option<String>,
    /// State code (required for US, CA, AU, BR)
    pub state: Option<String>,
    /// First line of address
    pub first_line: Option<String>,
}

/// User details containing personal information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
    /// First name
    pub first_name: Option<String>,
    /// Last name
    pub last_name: Option<String>,
    /// Phone number in international format
    pub phone_number: Option<String>,
    /// Date of birth (YYYY-MM-DD)
    pub date_of_birth: Option<String>,
    /// User's occupation
    pub occupation: Option<String>,
    /// Link to avatar image
    pub avatar: Option<String>,
    /// Primary address ID
    pub primary_address: Option<i64>,
    /// Address details
    pub address: Option<Address>,
}

/// Authenticated user information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// User ID
    pub id: i64,
    /// User's full name
    pub name: Option<String>,
    /// User's email
    pub email: Option<String>,
    /// Whether the user is active
    pub active: Option<bool>,
    /// User details (personal info, address, etc.)
    pub details: Option<UserDetails>,
}

/// Request to create a user with a registration code.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    /// New user's email address
    pub email: String,
    /// Unique registration code (at least 32 characters)
    pub registration_code: String,
    /// User default language (EN, US, PT, ES, FR, DE, IT, JA, RU, PL, HU, TR, RO, NL, HK)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// Request to check if a user exists.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserExistsRequest {
    /// Email to check
    pub email: String,
}

/// Response for user exists check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserExistsResponse {
    /// Whether the user exists
    pub exists: bool,
}

/// Contact email address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactEmail {
    /// Contact email address
    pub email: String,
}
