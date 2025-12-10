//! Contact API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{Contact, CreateContactRequest};

/// Contact API operations (write-only).
pub struct ContactsApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> ContactsApiMut<'a> {
    /// Create a contact from an identifier.
    ///
    /// Finds an existing discoverable Wise profile and adds it to the recipient list.
    /// The contact_id from the response can be used to create a transfer.
    ///
    /// # Arguments
    /// * `profile_id` - Your profile ID
    /// * `identifier` - Wisetag, email, or phone number
    /// * `target_currency` - 3-character currency code
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// # let client = wise_client::FullClient::new(wise_client::ClientConfig::with_token("test"))?;
    ///
    /// let contact = client.contacts().create(12345, "@JonathanP", "EUR").await?;
    /// println!("Added contact: {} ({})", contact.name, contact.contact_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        profile_id: i64,
        identifier: &str,
        target_currency: &str,
    ) -> Result<Contact> {
        let request = CreateContactRequest {
            identifier: identifier.to_string(),
            target_currency: target_currency.to_string(),
        };
        self.client
            .post(
                &format!(
                    "/v2/profiles/{}/contacts?isDirectIdentifierCreation=true",
                    profile_id
                ),
                &request,
            )
            .await
    }
}
