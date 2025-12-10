//! Contact models.

use serde::{Deserialize, Serialize};

/// Request to create a contact from identifier.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateContactRequest {
    /// Wise profile identifier (Wisetag, email, or phone).
    pub identifier: String,
    /// Target currency code.
    pub target_currency: String,
}

/// Response for creating a contact.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    /// Contact ID (UUID format).
    pub contact_id: String,
    /// Full name of the contact.
    pub name: String,
}
