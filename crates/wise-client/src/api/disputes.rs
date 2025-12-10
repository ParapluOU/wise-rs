//! Disputes API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{
    Dispute, DisputeReason, DisputeStatus, DisputesResponse, InitiateDisputeFlowRequest,
    WithdrawDisputeRequest,
};

/// Read-only Disputes API operations.
pub struct DisputesApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> DisputesApi<'a> {
    /// List disputes for a profile.
    ///
    /// # Arguments
    /// * `profile_id` - Profile ID
    /// * `status` - Optional status filter (ACTIVE or CLOSED)
    /// * `transaction_id` - Optional transaction ID filter
    /// * `page_size` - Items per page (10-100, default 10)
    /// * `page_number` - Page number (>= 1, default 1)
    pub async fn list(
        &self,
        profile_id: i64,
        status: Option<DisputeStatus>,
        transaction_id: Option<i64>,
        page_size: Option<u32>,
        page_number: Option<u32>,
    ) -> Result<DisputesResponse> {
        let mut url = format!("/v3/spend/profiles/{}/disputes?", profile_id);
        let mut params = Vec::new();

        if let Some(s) = status {
            let status_str = match s {
                DisputeStatus::Active => "ACTIVE",
                DisputeStatus::Closed => "CLOSED",
            };
            params.push(format!("status={}", status_str));
        }
        if let Some(tx_id) = transaction_id {
            params.push(format!("transactionId={}", tx_id));
        }
        if let Some(size) = page_size {
            params.push(format!("pageSize={}", size));
        }
        if let Some(page) = page_number {
            params.push(format!("pageNumber={}", page));
        }

        url.push_str(&params.join("&"));
        self.client.get(&url).await
    }

    /// Get a dispute by ID.
    pub async fn get(&self, profile_id: i64, dispute_id: &str) -> Result<Dispute> {
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/disputes/{}",
                profile_id, dispute_id
            ))
            .await
    }

    /// Get available dispute reasons.
    pub async fn get_reasons(&self, profile_id: i64) -> Result<Vec<DisputeReason>> {
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/dispute-form/reasons",
                profile_id
            ))
            .await
    }
}

/// Full Disputes API operations (includes write operations).
pub struct DisputesApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> DisputesApiMut<'a> {
    /// List disputes for a profile.
    pub async fn list(
        &self,
        profile_id: i64,
        status: Option<DisputeStatus>,
        transaction_id: Option<i64>,
        page_size: Option<u32>,
        page_number: Option<u32>,
    ) -> Result<DisputesResponse> {
        let mut url = format!("/v3/spend/profiles/{}/disputes?", profile_id);
        let mut params = Vec::new();

        if let Some(s) = status {
            let status_str = match s {
                DisputeStatus::Active => "ACTIVE",
                DisputeStatus::Closed => "CLOSED",
            };
            params.push(format!("status={}", status_str));
        }
        if let Some(tx_id) = transaction_id {
            params.push(format!("transactionId={}", tx_id));
        }
        if let Some(size) = page_size {
            params.push(format!("pageSize={}", size));
        }
        if let Some(page) = page_number {
            params.push(format!("pageNumber={}", page));
        }

        url.push_str(&params.join("&"));
        self.client.get(&url).await
    }

    /// Get a dispute by ID.
    pub async fn get(&self, profile_id: i64, dispute_id: &str) -> Result<Dispute> {
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/disputes/{}",
                profile_id, dispute_id
            ))
            .await
    }

    /// Get available dispute reasons.
    pub async fn get_reasons(&self, profile_id: i64) -> Result<Vec<DisputeReason>> {
        self.client
            .get(&format!(
                "/v3/spend/profiles/{}/dispute-form/reasons",
                profile_id
            ))
            .await
    }

    /// Initiate dispute flow (for dynamic flow integration).
    ///
    /// Returns JSON data to be used with Wise's Dynamic Flow framework.
    pub async fn initiate_flow(
        &self,
        profile_id: i64,
        scheme: &str,
        reason: &str,
        transaction_id: &str,
        email: &str,
    ) -> Result<serde_json::Value> {
        let request = InitiateDisputeFlowRequest {
            email: email.to_string(),
        };
        self.client
            .post(
                &format!(
                    "/v3/spend/profiles/{}/dispute-form/flows/step/{}/{}?transactionId={}",
                    profile_id, scheme, reason, transaction_id
                ),
                &request,
            )
            .await
    }

    /// Submit a dispute (for dynamic flow integration).
    pub async fn submit<T: serde::Serialize + ?Sized>(
        &self,
        profile_id: i64,
        scheme: &str,
        reason: &str,
        payload: &T,
    ) -> Result<serde_json::Value> {
        self.client
            .post(
                &format!(
                    "/v3/spend/profiles/{}/dispute-form/flows/{}/{}",
                    profile_id, scheme, reason
                ),
                payload,
            )
            .await
    }

    /// Withdraw a dispute.
    ///
    /// Can only withdraw if `can_withdraw` is true on the dispute.
    pub async fn withdraw(&self, profile_id: i64, dispute_id: &str) -> Result<Dispute> {
        let request = WithdrawDisputeRequest::new();
        self.client
            .put(
                &format!(
                    "/v3/spend/profiles/{}/disputes/{}/status",
                    profile_id, dispute_id
                ),
                &request,
            )
            .await
    }
}
