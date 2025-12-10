//! User API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::User;

/// User API operations.
pub struct UserApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> UserApi<'a> {
    /// Get the authenticated user's information.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// # let client = wise_client::ReadOnlyClient::new(wise_client::ClientConfig::with_token("test"))?;
    /// let user = client.user().get().await?;
    /// println!("User: {} ({})", user.name.unwrap_or_default(), user.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self) -> Result<User> {
        self.client.get("/v1/me").await
    }
}
