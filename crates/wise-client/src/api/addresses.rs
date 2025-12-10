//! Address API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{AddressRequirements, CreateAddressRequest, RegisteredAddress};

/// Read-only Address API operations.
pub struct AddressesApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> AddressesApi<'a> {
    /// Get an address by ID.
    pub async fn get(&self, address_id: i64) -> Result<RegisteredAddress> {
        self.client
            .get(&format!("/v1/addresses/{}", address_id))
            .await
    }

    /// List addresses for a profile.
    pub async fn list(&self, profile_id: i64) -> Result<Vec<RegisteredAddress>> {
        self.client
            .get(&format!("/v1/addresses?profile={}", profile_id))
            .await
    }

    /// Get address requirements.
    pub async fn get_requirements(&self) -> Result<Vec<AddressRequirements>> {
        self.client.get("/v1/address-requirements").await
    }
}

/// Full Address API operations (includes write operations).
pub struct AddressesApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> AddressesApiMut<'a> {
    /// Get an address by ID.
    pub async fn get(&self, address_id: i64) -> Result<RegisteredAddress> {
        self.client
            .get(&format!("/v1/addresses/{}", address_id))
            .await
    }

    /// List addresses for a profile.
    pub async fn list(&self, profile_id: i64) -> Result<Vec<RegisteredAddress>> {
        self.client
            .get(&format!("/v1/addresses?profile={}", profile_id))
            .await
    }

    /// Get address requirements.
    pub async fn get_requirements(&self) -> Result<Vec<AddressRequirements>> {
        self.client.get("/v1/address-requirements").await
    }

    /// Create or update an address.
    pub async fn create(&self, request: &CreateAddressRequest) -> Result<RegisteredAddress> {
        self.client.post("/v1/addresses", request).await
    }

    /// Post address requirements to discover additional fields.
    ///
    /// Use this when a field has `refresh_requirements_on_change=true`.
    pub async fn post_requirements(
        &self,
        request: &CreateAddressRequest,
    ) -> Result<Vec<AddressRequirements>> {
        self.client.post("/v1/address-requirements", request).await
    }
}
