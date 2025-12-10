//! User model.

use serde::{Deserialize, Serialize};

/// Authenticated user information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// User ID
    pub id: i64,
    /// User's name
    pub name: Option<String>,
    /// User's email
    pub email: Option<String>,
    /// Whether the user is active
    pub active: Option<bool>,
}
