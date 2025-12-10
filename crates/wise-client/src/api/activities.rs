//! Activity API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{ActivitiesResponse, ActivityResourceType, ActivityStatus};
use chrono::{DateTime, Utc};

/// Activity API operations (read-only).
pub struct ActivitiesApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> ActivitiesApi<'a> {
    /// List activities for a profile.
    ///
    /// # Arguments
    /// * `profile_id` - Profile ID
    /// * `params` - Optional filtering parameters
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> wise_client::error::Result<()> {
    /// # let client = wise_client::ReadOnlyClient::new(wise_client::ClientConfig::with_token("test"))?;
    /// let activities = client.activities().list(12345, None).await?;
    /// for activity in activities.activities {
    ///     println!("{}: {}", activity.activity_type, activity.title);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        profile_id: i64,
        params: Option<ListActivitiesParams>,
    ) -> Result<ActivitiesResponse> {
        let mut url = format!("/v1/profiles/{}/activities", profile_id);

        if let Some(p) = params {
            let mut query_parts = Vec::new();
            if let Some(ref rt) = p.monetary_resource_type {
                query_parts.push(format!(
                    "monetaryResourceType={}",
                    serde_json::to_string(rt).unwrap().trim_matches('"')
                ));
            }
            if let Some(ref status) = p.status {
                query_parts.push(format!(
                    "status={}",
                    serde_json::to_string(status).unwrap().trim_matches('"')
                ));
            }
            if let Some(ref since) = p.since {
                query_parts.push(format!("since={}", since.to_rfc3339()));
            }
            if let Some(ref until) = p.until {
                query_parts.push(format!("until={}", until.to_rfc3339()));
            }
            if let Some(ref cursor) = p.next_cursor {
                query_parts.push(format!("nextCursor={}", cursor));
            }
            if let Some(size) = p.size {
                query_parts.push(format!("size={}", size));
            }
            if !query_parts.is_empty() {
                url.push('?');
                url.push_str(&query_parts.join("&"));
            }
        }

        self.client.get(&url).await
    }
}

/// Parameters for listing activities.
#[derive(Debug, Clone, Default)]
pub struct ListActivitiesParams {
    /// Filter by resource type.
    pub monetary_resource_type: Option<ActivityResourceType>,
    /// Filter by status.
    pub status: Option<ActivityStatus>,
    /// Filter activities after this timestamp.
    pub since: Option<DateTime<Utc>>,
    /// Filter activities until this timestamp.
    pub until: Option<DateTime<Utc>>,
    /// Cursor for pagination.
    pub next_cursor: Option<String>,
    /// Page size (1-100, default 10).
    pub size: Option<u32>,
}
