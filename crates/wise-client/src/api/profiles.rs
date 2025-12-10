//! Profile API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::Profile;

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
        self.client.get(&format!("/v2/profiles/{}", profile_id)).await
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
        self.client.get(&format!("/v2/profiles/{}", profile_id)).await
    }

    // Write operations would go here (create, update, etc.)
}
