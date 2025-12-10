//! Profile models.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// User profile (personal or business).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// Profile ID
    pub id: i64,
    /// Profile type
    #[serde(rename = "type")]
    pub profile_type: ProfileType,
    /// Personal profile details
    #[serde(flatten)]
    pub details: ProfileDetails,
}

/// Type of profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProfileType {
    /// Personal profile
    Personal,
    /// Business profile
    Business,
}

/// Profile details (personal or business).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileDetails {
    /// First name (personal profiles)
    pub first_name: Option<String>,
    /// Last name (personal profiles)
    pub last_name: Option<String>,
    /// Date of birth (personal profiles)
    pub date_of_birth: Option<NaiveDate>,
    /// Phone number
    pub phone_number: Option<String>,
    /// Avatar URL
    pub avatar: Option<String>,
    /// Business name (business profiles)
    pub name: Option<String>,
    /// Registration number (business profiles)
    pub registration_number: Option<String>,
    /// Company type (business profiles)
    pub company_type: Option<String>,
    /// Company role
    pub company_role: Option<String>,
    /// Description of business
    pub description_of_business: Option<String>,
    /// Primary address
    pub primary_address: Option<i64>,
    /// Webpage
    pub webpage: Option<String>,
    /// Created timestamp
    pub created_at: Option<DateTime<Utc>>,
}

impl Profile {
    /// Returns the display name for this profile.
    pub fn display_name(&self) -> String {
        match self.profile_type {
            ProfileType::Personal => {
                let first = self.details.first_name.as_deref().unwrap_or("");
                let last = self.details.last_name.as_deref().unwrap_or("");
                format!("{} {}", first, last).trim().to_string()
            }
            ProfileType::Business => self
                .details
                .name
                .clone()
                .unwrap_or_else(|| "Business".to_string()),
        }
    }
}
