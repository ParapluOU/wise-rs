//! Card API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{
    Card, CardsResponse, SpendingPermissionsResponse, UpdateCardStatusRequest,
    UpdateSpendingPermissionRequest,
};

/// Read-only Card API operations.
pub struct CardsApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> CardsApi<'a> {
    /// List cards for a profile.
    ///
    /// # Arguments
    /// * `profile_id` - Profile ID
    /// * `page_size` - Items per page (10-100, default 10)
    /// * `page_number` - Page number (>= 1, default 1)
    pub async fn list(
        &self,
        profile_id: i64,
        page_size: Option<u32>,
        page_number: Option<u32>,
    ) -> Result<CardsResponse> {
        let size = page_size.unwrap_or(10);
        let page = page_number.unwrap_or(1);
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/cards?pageSize={}&pageNumber={}",
                profile_id, size, page
            ))
            .await
    }

    /// Get a card by token.
    pub async fn get(&self, profile_id: i64, card_token: &str) -> Result<Card> {
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/cards/{}",
                profile_id, card_token
            ))
            .await
    }

    /// Get spending permissions for a card.
    pub async fn get_permissions(
        &self,
        profile_id: i64,
        card_token: &str,
    ) -> Result<SpendingPermissionsResponse> {
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/cards/{}/spending-permissions",
                profile_id, card_token
            ))
            .await
    }
}

/// Full Card API operations (includes write operations).
pub struct CardsApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> CardsApiMut<'a> {
    /// List cards for a profile.
    pub async fn list(
        &self,
        profile_id: i64,
        page_size: Option<u32>,
        page_number: Option<u32>,
    ) -> Result<CardsResponse> {
        let size = page_size.unwrap_or(10);
        let page = page_number.unwrap_or(1);
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/cards?pageSize={}&pageNumber={}",
                profile_id, size, page
            ))
            .await
    }

    /// Get a card by token.
    pub async fn get(&self, profile_id: i64, card_token: &str) -> Result<Card> {
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/cards/{}",
                profile_id, card_token
            ))
            .await
    }

    /// Get spending permissions for a card.
    pub async fn get_permissions(
        &self,
        profile_id: i64,
        card_token: &str,
    ) -> Result<SpendingPermissionsResponse> {
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/cards/{}/spending-permissions",
                profile_id, card_token
            ))
            .await
    }

    /// Update card status.
    ///
    /// # Arguments
    /// * `status` - One of "ACTIVE", "FROZEN", or "BLOCKED"
    pub async fn update_status(
        &self,
        profile_id: i64,
        card_token: &str,
        status: &str,
    ) -> Result<Card> {
        let request = UpdateCardStatusRequest {
            status: status.to_string(),
        };
        self.client
            .put(
                &format!(
                    "/v3/spend/profiles/{}/cards/{}/status",
                    profile_id, card_token
                ),
                &request,
            )
            .await
    }

    /// Reset PIN entry count.
    ///
    /// Use when PIN entry tries are exceeded.
    pub async fn reset_pin_count(&self, profile_id: i64, card_token: &str) -> Result<()> {
        let empty: serde_json::Value = serde_json::json!({});
        let _: serde_json::Value = self
            .client
            .post(
                &format!(
                    "/v3/spend/profiles/{}/cards/{}/reset-pin-count",
                    profile_id, card_token
                ),
                &empty,
            )
            .await?;
        Ok(())
    }

    /// Update spending permission.
    pub async fn update_permission(
        &self,
        profile_id: i64,
        card_token: &str,
        request: &UpdateSpendingPermissionRequest,
    ) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .patch(
                &format!(
                    "/v3/spend/profiles/{}/cards/{}/spending-permissions",
                    profile_id, card_token
                ),
                request,
            )
            .await?;
        Ok(())
    }
}
