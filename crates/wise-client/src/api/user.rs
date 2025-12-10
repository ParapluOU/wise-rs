//! User API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{ContactEmail, CreateUserRequest, User, UserExistsRequest, UserExistsResponse};

/// User API operations (read-only).
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

    /// Get a user by their ID.
    ///
    /// # Arguments
    /// * `user_id` - The user's ID
    pub async fn get_by_id(&self, user_id: i64) -> Result<User> {
        self.client.get(&format!("/v1/users/{}", user_id)).await
    }

    /// Get the contact email for a user.
    ///
    /// # Arguments
    /// * `user_id` - The user's ID
    pub async fn get_contact_email(&self, user_id: i64) -> Result<ContactEmail> {
        self.client
            .get(&format!("/v1/users/{}/contact-email", user_id))
            .await
    }
}

/// User API operations (with write access).
pub struct UserApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> UserApiMut<'a> {
    /// Get the authenticated user's information.
    pub async fn get(&self) -> Result<User> {
        self.client.get("/v1/me").await
    }

    /// Get a user by their ID.
    pub async fn get_by_id(&self, user_id: i64) -> Result<User> {
        self.client.get(&format!("/v1/users/{}", user_id)).await
    }

    /// Get the contact email for a user.
    pub async fn get_contact_email(&self, user_id: i64) -> Result<ContactEmail> {
        self.client
            .get(&format!("/v1/users/{}/contact-email", user_id))
            .await
    }

    /// Create a new user with a registration code.
    ///
    /// This endpoint uses email as a unique identifier. If the email already exists,
    /// a 409 conflict error will be returned.
    ///
    /// # Arguments
    /// * `request` - The user creation request containing email, registration code, and optional language
    pub async fn create(&self, request: &CreateUserRequest) -> Result<User> {
        self.client
            .post("/v1/user/signup/registration_code", request)
            .await
    }

    /// Check if a user with the given email exists.
    ///
    /// # Arguments
    /// * `email` - The email address to check
    pub async fn exists(&self, email: &str) -> Result<UserExistsResponse> {
        let request = UserExistsRequest {
            email: email.to_string(),
        };
        self.client.post("/v1/users/exists", &request).await
    }

    /// Set the contact email for a user.
    ///
    /// The contact email is used for notifications when a user was registered
    /// with a dummy email address.
    ///
    /// # Arguments
    /// * `user_id` - The user's ID
    /// * `email` - The new contact email address
    pub async fn set_contact_email(&self, user_id: i64, email: &str) -> Result<ContactEmail> {
        let request = ContactEmail {
            email: email.to_string(),
        };
        self.client
            .put(&format!("/v1/users/{}/contact-email", user_id), &request)
            .await
    }
}
