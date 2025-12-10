//! Error types for the Wise client.

use thiserror::Error;

/// Errors that can occur when using the Wise client.
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP transport error
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// API returned an error response
    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
        errors: Vec<ApiFieldError>,
    },

    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded, retry after {retry_after_secs:?} seconds")]
    RateLimit { retry_after_secs: Option<u64> },

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// JSON serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// URL parsing error
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),
}

/// Field-level error from the Wise API.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiFieldError {
    /// Error code
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Field path that caused the error
    pub path: Option<String>,
    /// Additional arguments
    #[serde(default)]
    pub arguments: Vec<serde_json::Value>,
}

/// Result type alias for Wise client operations.
pub type Result<T> = std::result::Result<T, Error>;

/// API error response format from Wise.
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum ApiErrorResponse {
    /// Array of field errors
    Errors { errors: Vec<ApiFieldError> },
    /// Simple error with message
    Simple { error: String, error_description: Option<String> },
}

impl From<ApiErrorResponse> for (String, Vec<ApiFieldError>) {
    fn from(resp: ApiErrorResponse) -> Self {
        match resp {
            ApiErrorResponse::Errors { errors } => {
                let message = errors
                    .first()
                    .map(|e| e.message.clone())
                    .unwrap_or_else(|| "Unknown error".to_string());
                (message, errors)
            }
            ApiErrorResponse::Simple { error, error_description } => {
                let message = error_description.unwrap_or(error);
                (message, vec![])
            }
        }
    }
}
