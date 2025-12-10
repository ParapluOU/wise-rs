//! 3D Secure authentication API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{ChallengeResultRequest, ChallengeStatus};

/// 3D Secure API operations (write-only).
pub struct ThreeDSecureApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> ThreeDSecureApiMut<'a> {
    /// Inform Wise of a 3DS challenge result.
    ///
    /// Call this after the customer has accepted or rejected a push notification
    /// for a 3DS challenge. Must be called before the expiration time from the
    /// webhook event.
    ///
    /// # Arguments
    /// * `profile_id` - Profile ID
    /// * `transaction_reference` - Transaction reference from webhook event
    /// * `status` - Whether the challenge was APPROVED or REJECTED
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// use wise_client::models::ChallengeStatus;
    /// # let client = wise_client::FullClient::new(wise_client::ClientConfig::with_token("test"))?;
    ///
    /// client.three_d_secure().inform_challenge_result(
    ///     12345,
    ///     "148579538",
    ///     ChallengeStatus::Approved,
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn inform_challenge_result(
        &self,
        profile_id: i64,
        transaction_reference: &str,
        status: ChallengeStatus,
    ) -> Result<()> {
        let request = ChallengeResultRequest {
            transaction_reference: transaction_reference.to_string(),
            challenge_status: status,
        };
        let _: serde_json::Value = self
            .client
            .post(
                &format!("/v3/spend/profiles/{}/3dsecure/challenge-result", profile_id),
                &request,
            )
            .await?;
        Ok(())
    }
}
