//! Profile models.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Profile address.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileAddress {
    /// Address ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// First line of address
    pub address_first_line: String,
    /// City
    pub city: String,
    /// 2-letter country code (e.g., "GB")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_iso2_code: Option<String>,
    /// 3-letter country code (e.g., "gbr") - lowercase
    pub country_iso3_code: String,
    /// Postal/ZIP code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    /// State code (required for US, CA, BR, AU)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
}

/// Contact details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactDetails {
    /// Contact email address
    pub email: String,
    /// Contact phone number in international format (e.g., "+1408XXXXXXX")
    pub phone_number: String,
}

/// Occupation information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Occupation {
    /// Occupation code (free-form text like "Software Engineer")
    pub code: String,
    /// Occupation format (currently only "FREE_FORM")
    pub format: String,
}

impl Occupation {
    /// Create a new free-form occupation.
    pub fn free_form(occupation: impl Into<String>) -> Self {
        Self {
            code: occupation.into(),
            format: "FREE_FORM".to_string(),
        }
    }
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

/// Profile state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProfileState {
    /// Profile is visible and active
    Visible,
    /// Profile is hidden
    Hidden,
    /// Profile is deactivated
    Deactivated,
}

/// Company type for business profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CompanyType {
    Limited,
    Partnership,
    SoleTrader,
    LimitedByGuarantee,
    LimitedLiabilityCompany,
    ForProfitCorporation,
    NonProfitCorporation,
    LimitedPartnership,
    LimitedLiabilityPartnership,
    GeneralPartnership,
    SoleProprietorship,
    PrivateLimitedCompany,
    PublicLimitedCompany,
    Trust,
    Other,
}

/// Company role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CompanyRole {
    Owner,
    Director,
    Other,
}

/// User profile (personal or business) - v2 API response format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// Profile ID
    pub id: i64,
    /// Public identifier for the profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_id: Option<String>,
    /// User ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Profile type
    #[serde(rename = "type")]
    pub profile_type: ProfileType,
    /// Main registered address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<ProfileAddress>,
    /// Primary email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Created timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    /// Updated timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    /// Avatar URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Current state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_state: Option<ProfileState>,
    /// Contact details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_details: Option<ContactDetails>,
    /// Full name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,

    // Personal profile fields
    /// First name (personal profiles)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name (personal profiles)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Preferred name (personal profiles)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_name: Option<String>,
    /// Date of birth (personal profiles)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<NaiveDate>,
    /// Phone number (personal profiles)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    // Business profile fields
    /// Business name (business profiles)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// Registration number (business profiles)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    /// Description of business (business profiles)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_of_business: Option<String>,
    /// Business webpage (business profiles)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webpage: Option<String>,
    /// Company type (business profiles)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_type: Option<CompanyType>,
    /// Free-form business description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_free_form_description: Option<String>,
    /// Primary business category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_level_category: Option<String>,
    /// Secondary business category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_level_category: Option<String>,
    /// Operational addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_addresses: Option<Vec<ProfileAddress>>,
    /// Secondary addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_addresses: Option<Vec<ProfileAddress>>,
}

impl Profile {
    /// Returns the display name for this profile.
    pub fn display_name(&self) -> String {
        if let Some(full_name) = &self.full_name {
            return full_name.clone();
        }

        match self.profile_type {
            ProfileType::Personal => {
                let first = self.first_name.as_deref().unwrap_or("");
                let last = self.last_name.as_deref().unwrap_or("");
                format!("{} {}", first, last).trim().to_string()
            }
            ProfileType::Business => self
                .business_name
                .clone()
                .unwrap_or_else(|| "Business".to_string()),
        }
    }
}

/// Request to create a personal profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePersonalProfile {
    /// First name (including middle names), max 30 chars
    pub first_name: String,
    /// Last name, max 30 chars
    pub last_name: String,
    /// Preferred first name if different from legal first name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_name: Option<String>,
    /// First name in Katakana (required for from-JPY personal transfers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_in_kana: Option<String>,
    /// Last name in Katakana (required for from-JPY personal transfers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_in_kana: Option<String>,
    /// Address
    pub address: ProfileAddress,
    /// 3-letter nationality code (lowercase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// Date of birth
    pub date_of_birth: NaiveDate,
    /// External customer ID for reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_customer_id: Option<String>,
    /// Contact details
    pub contact_details: ContactDetails,
    /// Occupations (required for CA, IN, JP, ID, IL, MX and US-NM)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupations: Option<Vec<Occupation>>,
}

/// Request to create a business profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBusinessProfile {
    /// Business name
    pub business_name: String,
    /// Business name in Katakana (only for Japanese businesses)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name_in_katakana: Option<String>,
    /// Business free form description (required if companyType is OTHER)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_free_form_description: Option<String>,
    /// Business registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    /// Australian Company Number (only for Australian businesses)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acn: Option<String>,
    /// Australian Business Number (only for Australian businesses)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abn: Option<String>,
    /// Australian Registered Body Number (only for Australian businesses)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arbn: Option<String>,
    /// Company legal form
    pub company_type: CompanyType,
    /// Role of person
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_role: Option<CompanyRole>,
    /// Registered address
    pub address: ProfileAddress,
    /// External customer ID for reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_customer_id: Option<String>,
    /// Email of the actor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_email: Option<String>,
    /// Primary business category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_level_category: Option<String>,
    /// Secondary business category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_level_category: Option<String>,
    /// Operational addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_addresses: Option<Vec<ProfileAddress>>,
    /// Business webpage (required if companyType is OTHER)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webpage: Option<String>,
    /// Business representative details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_representative: Option<BusinessRepresentative>,
}

/// Business representative details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessRepresentative {
    /// ID of an existing business representative
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_representative_id: Option<i64>,
    /// First name (including middle names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Preferred first name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_name: Option<String>,
    /// Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<ProfileAddress>,
    /// Date of birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<NaiveDate>,
    /// Contact details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_details: Option<ContactDetails>,
}

/// Request to update a personal profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePersonalProfile {
    /// First name (including middle names), max 30 chars
    pub first_name: String,
    /// Last name, max 30 chars
    pub last_name: String,
    /// Preferred first name if different from legal first name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_name: Option<String>,
    /// First name in Katakana
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_in_kana: Option<String>,
    /// Last name in Katakana
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_in_kana: Option<String>,
    /// Address
    pub address: ProfileAddress,
    /// 3-letter nationality code (lowercase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// Date of birth
    pub date_of_birth: NaiveDate,
    /// Contact details
    pub contact_details: ContactDetails,
    /// Occupations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupations: Option<Vec<Occupation>>,
}

/// Request to update a business profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBusinessProfile {
    /// Business profile ID (must match URL parameter)
    pub id: String,
    /// Business name
    pub business_name: String,
    /// Business name in Katakana
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name_in_katakana: Option<String>,
    /// Business free form description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_free_form_description: Option<String>,
    /// Business registration number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    /// Australian Company Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acn: Option<String>,
    /// Australian Business Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abn: Option<String>,
    /// Australian Registered Body Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arbn: Option<String>,
    /// Company legal form
    pub company_type: CompanyType,
    /// Registered address
    pub address: ProfileAddress,
    /// Primary business category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_level_category: Option<String>,
    /// Secondary business category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_level_category: Option<String>,
    /// Operational addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_addresses: Option<Vec<ProfileAddress>>,
    /// Business webpage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webpage: Option<String>,
}

/// Business director.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Director {
    /// Director ID (auto-generated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// First name
    pub first_name: String,
    /// Last name
    pub last_name: String,
    /// Date of birth
    pub date_of_birth: NaiveDate,
    /// 3-letter country of residence code
    pub country_of_residence_iso3_code: String,
}

/// Ultimate beneficial owner (UBO).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltimateBeneficialOwner {
    /// UBO ID (auto-generated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Full name
    pub name: String,
    /// Date of birth
    pub date_of_birth: NaiveDate,
    /// 3-letter country of residence code
    pub country_of_residence_iso3_code: String,
    /// First line of address
    pub address_first_line: String,
    /// Post code
    pub post_code: String,
    /// Percentage of ownership (can be null in some cases)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_percentage: Option<i32>,
}

/// Verification document type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationDocumentType {
    DriversLicence,
    IdentityCard,
    GreenCard,
    MyNumber,
    Passport,
    Ssn,
    EmiratesEmployer,
    EmiratesPlaceOfBirth,
    CpfCnpj,
    FinancialCapacityBr,
    Other,
}

/// Request to create a verification document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVerificationDocument {
    /// Person first name in document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Person last name in document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Document type
    #[serde(rename = "type")]
    pub document_type: VerificationDocumentType,
    /// Document number or value (max 30 chars, digits only for SSN)
    pub unique_identifier: String,
    /// Document issue date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<NaiveDate>,
    /// Issuing country code (e.g., "US")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_country: Option<String>,
    /// Issuing state code (e.g., "NY")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_state: Option<String>,
    /// Document expiry date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<NaiveDate>,
    /// 2-letter nationality code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// Employer name (for EMIRATES_EMPLOYER type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer_name: Option<String>,
    /// Employer city (for EMIRATES_EMPLOYER type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer_city: Option<String>,
    /// Employer country (for EMIRATES_EMPLOYER type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer_country: Option<String>,
    /// Birth city (for EMIRATES_PLACE_OF_BIRTH type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_city: Option<String>,
    /// Birth country (for EMIRATES_PLACE_OF_BIRTH type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_country: Option<String>,
}

/// Response from creating a verification document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationDocumentResponse {
    /// Error message if any
    pub error_message: Option<String>,
    /// Whether the operation was successful
    pub success: bool,
}

/// Verification status for a currency route.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationRouteStatus {
    /// Source currency code
    pub source_currency: String,
    /// Maximum entitled amount
    pub maximum_entitled_amount: f64,
    /// Current verification status
    pub current_status: VerificationStatus,
}

/// Verification status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    /// Profile is verified
    Verified,
    /// Profile is not verified
    NotVerified,
}

/// Response from verification status check.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationStatusResponse {
    /// Status for each currency route
    pub routes: Vec<VerificationRouteStatus>,
    /// Request ID
    pub request_id: String,
}
