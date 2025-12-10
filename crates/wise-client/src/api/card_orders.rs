//! Card order API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{
    CardOrder, CardOrderRequirementsResponse, CardOrdersResponse, CardProgramsResponse,
    CreateCardOrderRequest, UpdateCardOrderStatusRequest, ValidateAddressRequest,
};

/// Read-only Card Order API operations.
pub struct CardOrdersApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> CardOrdersApi<'a> {
    /// Get a card order by ID.
    pub async fn get(&self, profile_id: i64, card_order_id: i64) -> Result<CardOrder> {
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/card-orders/{}",
                profile_id, card_order_id
            ))
            .await
    }

    /// List card orders for a profile.
    pub async fn list(
        &self,
        profile_id: i64,
        page_size: Option<u32>,
        page_number: Option<u32>,
    ) -> Result<CardOrdersResponse> {
        let size = page_size.unwrap_or(10);
        let page = page_number.unwrap_or(1);
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/card-orders?pageSize={}&pageNumber={}",
                profile_id, size, page
            ))
            .await
    }

    /// Get available card programs.
    pub async fn get_availability(&self, profile_id: i64) -> Result<CardProgramsResponse> {
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/card-orders/availability",
                profile_id
            ))
            .await
    }

    /// Get requirements for a card order.
    pub async fn get_requirements(
        &self,
        profile_id: i64,
        card_order_id: i64,
    ) -> Result<CardOrderRequirementsResponse> {
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/card-orders/{}/requirements",
                profile_id, card_order_id
            ))
            .await
    }
}

/// Full Card Order API operations (includes write operations).
pub struct CardOrdersApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> CardOrdersApiMut<'a> {
    /// Get a card order by ID.
    pub async fn get(&self, profile_id: i64, card_order_id: i64) -> Result<CardOrder> {
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/card-orders/{}",
                profile_id, card_order_id
            ))
            .await
    }

    /// List card orders for a profile.
    pub async fn list(
        &self,
        profile_id: i64,
        page_size: Option<u32>,
        page_number: Option<u32>,
    ) -> Result<CardOrdersResponse> {
        let size = page_size.unwrap_or(10);
        let page = page_number.unwrap_or(1);
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/card-orders?pageSize={}&pageNumber={}",
                profile_id, size, page
            ))
            .await
    }

    /// Get available card programs.
    pub async fn get_availability(&self, profile_id: i64) -> Result<CardProgramsResponse> {
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/card-orders/availability",
                profile_id
            ))
            .await
    }

    /// Get requirements for a card order.
    pub async fn get_requirements(
        &self,
        profile_id: i64,
        card_order_id: i64,
    ) -> Result<CardOrderRequirementsResponse> {
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/card-orders/{}/requirements",
                profile_id, card_order_id
            ))
            .await
    }

    /// Create a card order.
    ///
    /// Requires `X-idempotence-uuid` header for retries.
    pub async fn create(
        &self,
        profile_id: i64,
        request: &CreateCardOrderRequest,
    ) -> Result<CardOrder> {
        self.client
            .post(
                &format!("/v3/spend/profiles/{}/card-orders", profile_id),
                request,
            )
            .await
    }

    /// Update card order status.
    ///
    /// Can be set to CANCELLED (or COMPLETED, deprecated).
    pub async fn update_status(
        &self,
        profile_id: i64,
        card_order_id: i64,
        status: &str,
    ) -> Result<()> {
        let request = UpdateCardOrderStatusRequest {
            status: status.to_string(),
        };
        let _: serde_json::Value = self
            .client
            .put(
                &format!(
                    "/v3/spend/profiles/{}/card-orders/{}/status",
                    profile_id, card_order_id
                ),
                &request,
            )
            .await?;
        Ok(())
    }

    /// Validate an address for card ordering.
    pub async fn validate_address(&self, request: &ValidateAddressRequest) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .post("/v3/spend/address/validate", request)
            .await?;
        Ok(())
    }
}
