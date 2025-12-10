//! Partner cases API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{AddCaseCommentRequest, CaseCommentsResponse, CreateCaseRequest, PartnerCase};

/// Read-only Partner Cases API operations.
pub struct CasesApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> CasesApi<'a> {
    /// Get a partner case by ID.
    pub async fn get(&self, case_id: i64) -> Result<PartnerCase> {
        self.client.get(&format!("/v1/cases/{}", case_id)).await
    }

    /// Get comments for a partner case.
    ///
    /// Comments are returned newest first.
    pub async fn get_comments(&self, case_id: i64) -> Result<CaseCommentsResponse> {
        self.client
            .get(&format!("/v1/cases/{}/comments", case_id))
            .await
    }
}

/// Full Partner Cases API operations (includes write operations).
pub struct CasesApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> CasesApiMut<'a> {
    /// Get a partner case by ID.
    pub async fn get(&self, case_id: i64) -> Result<PartnerCase> {
        self.client.get(&format!("/v1/cases/{}", case_id)).await
    }

    /// Get comments for a partner case.
    pub async fn get_comments(&self, case_id: i64) -> Result<CaseCommentsResponse> {
        self.client
            .get(&format!("/v1/cases/{}/comments", case_id))
            .await
    }

    /// Create a new partner case.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// use wise_client::models::{CreateCaseRequest, CaseType, CaseDetails};
    /// # let client = wise_client::FullClient::new(wise_client::ClientConfig::with_token("test"))?;
    ///
    /// let case = client.cases().create(&CreateCaseRequest {
    ///     case_type: CaseType::GeneralEnquiry,
    ///     subject: "Inquiry about Transfer 12345".to_string(),
    ///     details: CaseDetails {
    ///         transfer_id: Some(58114690),
    ///         profile_id: Some(14556049),
    ///         user_id: 1234,
    ///     },
    ///     external_id: Some("partner_12345".to_string()),
    ///     description: "Initial summary of the issue".to_string(),
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, request: &CreateCaseRequest) -> Result<PartnerCase> {
        self.client.post("/v1/cases", request).await
    }

    /// Add a comment to a partner case.
    pub async fn add_comment(&self, case_id: i64, comment: &str) -> Result<()> {
        let request = AddCaseCommentRequest {
            comment: comment.to_string(),
        };
        let _: serde_json::Value = self
            .client
            .put(&format!("/v1/cases/{}/comments", case_id), &request)
            .await?;
        Ok(())
    }
}
