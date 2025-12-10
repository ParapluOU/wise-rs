//! Batch group API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{
    BatchGroup, BatchPaymentResponse, BatchPaymentType, CreateBatchGroupRequest,
    FundBatchDirectDebitRequest, FundBatchGroupRequest, PaymentInitiation, Transfer,
    UpdateBatchGroupRequest,
};
use uuid::Uuid;

/// Read-only Batch Group API operations.
pub struct BatchGroupsApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> BatchGroupsApi<'a> {
    /// Get a batch group by ID.
    pub async fn get(&self, profile_id: i64, batch_group_id: Uuid) -> Result<BatchGroup> {
        self.client
            .get(&format!(
                "/v3/profiles/{}/batch-groups/{}",
                profile_id, batch_group_id
            ))
            .await
    }

    /// Get a payment initiation by ID.
    pub async fn get_payment_initiation(
        &self,
        profile_id: i64,
        batch_group_id: Uuid,
        payment_initiation_id: i64,
    ) -> Result<PaymentInitiation> {
        self.client
            .get(&format!(
                "/v1/profiles/{}/batch-groups/{}/payment-initiations/{}",
                profile_id, batch_group_id, payment_initiation_id
            ))
            .await
    }
}

/// Full Batch Group API operations (includes write operations).
pub struct BatchGroupsApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> BatchGroupsApiMut<'a> {
    /// Get a batch group by ID.
    pub async fn get(&self, profile_id: i64, batch_group_id: Uuid) -> Result<BatchGroup> {
        self.client
            .get(&format!(
                "/v3/profiles/{}/batch-groups/{}",
                profile_id, batch_group_id
            ))
            .await
    }

    /// Create a new batch group.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// use wise_client::models::CreateBatchGroupRequest;
    /// # let client = wise_client::FullClient::new(wise_client::ClientConfig::with_token("test"))?;
    ///
    /// let batch = client.batch_groups().create(12345, &CreateBatchGroupRequest {
    ///     source_currency: "GBP".to_string(),
    ///     name: "Monthly payroll".to_string(),
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        profile_id: i64,
        request: &CreateBatchGroupRequest,
    ) -> Result<BatchGroup> {
        self.client
            .post(&format!("/v3/profiles/{}/batch-groups", profile_id), request)
            .await
    }

    /// Complete a batch group.
    ///
    /// Closes the batch group to further changes and enables funding.
    pub async fn complete(
        &self,
        profile_id: i64,
        batch_group_id: Uuid,
        version: i64,
    ) -> Result<BatchGroup> {
        let request = UpdateBatchGroupRequest {
            status: "COMPLETED".to_string(),
            version,
        };
        self.client
            .patch(
                &format!("/v3/profiles/{}/batch-groups/{}", profile_id, batch_group_id),
                &request,
            )
            .await
    }

    /// Cancel a batch group.
    ///
    /// Only batches that are not funded can be cancelled.
    pub async fn cancel(
        &self,
        profile_id: i64,
        batch_group_id: Uuid,
        version: i64,
    ) -> Result<BatchGroup> {
        let request = UpdateBatchGroupRequest {
            status: "CANCELLED".to_string(),
            version,
        };
        self.client
            .patch(
                &format!("/v3/profiles/{}/batch-groups/{}", profile_id, batch_group_id),
                &request,
            )
            .await
    }

    /// Create a transfer in a batch group.
    ///
    /// Uses the same request format as regular transfer creation.
    pub async fn create_transfer<T: serde::Serialize + ?Sized>(
        &self,
        profile_id: i64,
        batch_group_id: Uuid,
        request: &T,
    ) -> Result<Transfer> {
        self.client
            .post(
                &format!(
                    "/v3/profiles/{}/batch-groups/{}/transfers",
                    profile_id, batch_group_id
                ),
                request,
            )
            .await
    }

    /// Fund a batch group from balance.
    ///
    /// The batch group must be completed first.
    pub async fn fund(
        &self,
        profile_id: i64,
        batch_group_id: Uuid,
    ) -> Result<BatchPaymentResponse> {
        let request = FundBatchGroupRequest {
            payment_type: BatchPaymentType::Balance,
        };
        self.client
            .post(
                &format!(
                    "/v3/profiles/{}/batch-payments/{}/payments",
                    profile_id, batch_group_id
                ),
                &request,
            )
            .await
    }

    /// Fund a batch group via direct debit.
    pub async fn fund_direct_debit(
        &self,
        profile_id: i64,
        batch_group_id: Uuid,
        account_id: i64,
        reference: Option<String>,
    ) -> Result<PaymentInitiation> {
        let request = FundBatchDirectDebitRequest {
            payment_type: "DIRECT_DEBIT".to_string(),
            account_id,
            reference,
        };
        self.client
            .post(
                &format!(
                    "/v1/profiles/{}/batch-groups/{}/payment-initiations",
                    profile_id, batch_group_id
                ),
                &request,
            )
            .await
    }

    /// Get a payment initiation by ID.
    pub async fn get_payment_initiation(
        &self,
        profile_id: i64,
        batch_group_id: Uuid,
        payment_initiation_id: i64,
    ) -> Result<PaymentInitiation> {
        self.client
            .get(&format!(
                "/v1/profiles/{}/batch-groups/{}/payment-initiations/{}",
                profile_id, batch_group_id, payment_initiation_id
            ))
            .await
    }
}
