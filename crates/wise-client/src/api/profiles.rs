//! Profile API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{
    BusinessRepresentative, CreateBusinessProfile, CreatePersonalProfile, CreateVerificationDocument,
    Director, Profile, UltimateBeneficialOwner, UpdateBusinessProfile, UpdatePersonalProfile,
    VerificationDocumentResponse, VerificationStatusResponse,
};

/// Read-only Profile API operations.
pub struct ProfilesApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> ProfilesApi<'a> {
    /// List all profiles for the authenticated user.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// # let client = wise_client::ReadOnlyClient::new(wise_client::ClientConfig::with_token("test"))?;
    /// let profiles = client.profiles().list().await?;
    /// for profile in profiles {
    ///     println!("{}: {}", profile.id, profile.display_name());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<Vec<Profile>> {
        self.client.get("/v2/profiles").await
    }

    /// Get a specific profile by ID.
    pub async fn get(&self, profile_id: i64) -> Result<Profile> {
        self.client
            .get(&format!("/v2/profiles/{}", profile_id))
            .await
    }

    /// Get the business representative for a business profile.
    pub async fn get_business_representative(
        &self,
        profile_id: i64,
    ) -> Result<BusinessRepresentative> {
        self.client
            .get(&format!(
                "/v3/profiles/{}/business-profile/business-representative",
                profile_id
            ))
            .await
    }

    /// List directors for a business profile.
    pub async fn list_directors(&self, profile_id: i64) -> Result<Vec<Director>> {
        self.client
            .get(&format!("/v1/profiles/{}/directors", profile_id))
            .await
    }

    /// List ultimate beneficial owners (UBOs) for a business profile.
    pub async fn list_ubos(&self, profile_id: i64) -> Result<Vec<UltimateBeneficialOwner>> {
        self.client
            .get(&format!("/v1/profiles/{}/ubos", profile_id))
            .await
    }

    /// Check the verification status of a profile for specific source currencies.
    ///
    /// # Arguments
    /// * `profile_id` - The profile ID to check
    /// * `source_currencies` - List of currency codes to check (e.g., ["GBP", "USD"])
    pub async fn get_verification_status(
        &self,
        profile_id: i64,
        source_currencies: &[&str],
    ) -> Result<VerificationStatusResponse> {
        let currencies = source_currencies.join(",");
        self.client
            .post(
                &format!(
                    "/v3/profiles/{}/verification-status/bank-transfer?source_currencies={}",
                    profile_id, currencies
                ),
                &serde_json::Value::Null,
            )
            .await
    }
}

/// Full Profile API operations (includes write operations).
pub struct ProfilesApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> ProfilesApiMut<'a> {
    /// List all profiles for the authenticated user.
    pub async fn list(&self) -> Result<Vec<Profile>> {
        self.client.get("/v2/profiles").await
    }

    /// Get a specific profile by ID.
    pub async fn get(&self, profile_id: i64) -> Result<Profile> {
        self.client
            .get(&format!("/v2/profiles/{}", profile_id))
            .await
    }

    /// Create a personal profile.
    ///
    /// # Arguments
    /// * `request` - Personal profile creation request
    /// * `idempotence_uuid` - Optional unique ID for idempotency
    pub async fn create_personal(&self, request: &CreatePersonalProfile) -> Result<Profile> {
        self.client
            .post("/v2/profiles/personal-profile", request)
            .await
    }

    /// Create a business profile (v3 API).
    ///
    /// # Arguments
    /// * `request` - Business profile creation request with business representative details
    pub async fn create_business(&self, request: &CreateBusinessProfile) -> Result<Profile> {
        self.client
            .post("/v3/profiles/business-profile", request)
            .await
    }

    /// Update a personal profile.
    ///
    /// If the profile has been verified, there are restrictions on what can be changed.
    /// Use `open_update_window` first if needed.
    pub async fn update_personal(
        &self,
        profile_id: i64,
        request: &UpdatePersonalProfile,
    ) -> Result<Profile> {
        self.client
            .put(
                &format!("/v2/profiles/{}/personal-profile", profile_id),
                request,
            )
            .await
    }

    /// Update a business profile.
    ///
    /// If the profile has been verified, there are restrictions on what can be changed.
    /// Use `open_update_window` first if needed.
    pub async fn update_business(
        &self,
        profile_id: i64,
        request: &UpdateBusinessProfile,
    ) -> Result<Profile> {
        self.client
            .put(
                &format!("/v2/profiles/{}/business-profile", profile_id),
                request,
            )
            .await
    }

    /// Get the business representative for a business profile.
    pub async fn get_business_representative(
        &self,
        profile_id: i64,
    ) -> Result<BusinessRepresentative> {
        self.client
            .get(&format!(
                "/v3/profiles/{}/business-profile/business-representative",
                profile_id
            ))
            .await
    }

    /// Update the business representative for a business profile.
    pub async fn update_business_representative(
        &self,
        profile_id: i64,
        request: &BusinessRepresentative,
    ) -> Result<BusinessRepresentative> {
        self.client
            .put(
                &format!(
                    "/v3/profiles/{}/business-profile/business-representative",
                    profile_id
                ),
                request,
            )
            .await
    }

    /// Create a verification document for a personal profile.
    pub async fn create_verification_document(
        &self,
        profile_id: i64,
        request: &CreateVerificationDocument,
    ) -> Result<VerificationDocumentResponse> {
        self.client
            .post(
                &format!("/v1/profiles/{}/verification-documents", profile_id),
                request,
            )
            .await
    }

    /// List directors for a business profile.
    pub async fn list_directors(&self, profile_id: i64) -> Result<Vec<Director>> {
        self.client
            .get(&format!("/v1/profiles/{}/directors", profile_id))
            .await
    }

    /// Add directors to a business profile.
    pub async fn add_directors(
        &self,
        profile_id: i64,
        directors: &[Director],
    ) -> Result<Vec<Director>> {
        self.client
            .post(&format!("/v1/profiles/{}/directors", profile_id), directors)
            .await
    }

    /// Update (replace) all directors for a business profile.
    pub async fn update_directors(
        &self,
        profile_id: i64,
        directors: &[Director],
    ) -> Result<Vec<Director>> {
        self.client
            .put(&format!("/v1/profiles/{}/directors", profile_id), directors)
            .await
    }

    /// List ultimate beneficial owners (UBOs) for a business profile.
    pub async fn list_ubos(&self, profile_id: i64) -> Result<Vec<UltimateBeneficialOwner>> {
        self.client
            .get(&format!("/v1/profiles/{}/ubos", profile_id))
            .await
    }

    /// Add UBOs to a business profile.
    pub async fn add_ubos(
        &self,
        profile_id: i64,
        ubos: &[UltimateBeneficialOwner],
    ) -> Result<Vec<UltimateBeneficialOwner>> {
        self.client
            .post(&format!("/v1/profiles/{}/ubos", profile_id), ubos)
            .await
    }

    /// Update (replace) all UBOs for a business profile.
    pub async fn update_ubos(
        &self,
        profile_id: i64,
        ubos: &[UltimateBeneficialOwner],
    ) -> Result<Vec<UltimateBeneficialOwner>> {
        self.client
            .put(&format!("/v1/profiles/{}/ubos", profile_id), ubos)
            .await
    }

    /// Open an update window for a profile.
    ///
    /// This is required before updating certain profile fields that are normally locked.
    pub async fn open_update_window(&self, profile_id: i64) -> Result<()> {
        self.client
            .post::<serde_json::Value, _>(
                &format!("/v1/profiles/{}/update-window", profile_id),
                &serde_json::Value::Null,
            )
            .await?;
        Ok(())
    }

    /// Close an update window for a profile.
    pub async fn close_update_window(&self, profile_id: i64) -> Result<()> {
        self.client
            .delete(&format!("/v1/profiles/{}/update-window", profile_id))
            .await
    }

    /// Check the verification status of a profile for specific source currencies.
    pub async fn get_verification_status(
        &self,
        profile_id: i64,
        source_currencies: &[&str],
    ) -> Result<VerificationStatusResponse> {
        let currencies = source_currencies.join(",");
        self.client
            .post(
                &format!(
                    "/v3/profiles/{}/verification-status/bank-transfer?source_currencies={}",
                    profile_id, currencies
                ),
                &serde_json::Value::Null,
            )
            .await
    }
}
