//! Configuration types for the Wise client.

use secrecy::SecretString;
use std::time::Duration;

/// Environment configuration for the Wise API.
#[derive(Debug, Clone, Default)]
pub enum Environment {
    /// Production API (https://api.wise.com)
    Production,
    /// Sandbox API for testing (https://api.wise-sandbox.com)
    #[default]
    Sandbox,
}

impl Environment {
    /// Returns the base URL for this environment.
    pub fn base_url(&self) -> &str {
        match self {
            Environment::Production => "https://api.wise.com",
            Environment::Sandbox => "https://api.wise-sandbox.com",
        }
    }
}

/// Authentication configuration.
#[derive(Clone)]
pub enum AuthConfig {
    /// Personal API token
    ApiToken(SecretString),
}

impl std::fmt::Debug for AuthConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthConfig::ApiToken(_) => f.debug_tuple("ApiToken").field(&"[REDACTED]").finish(),
        }
    }
}

/// Client configuration.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// API environment (production or sandbox)
    pub environment: Environment,
    /// Authentication configuration
    pub auth: AuthConfig,
    /// Request timeout
    pub timeout: Duration,
    /// Custom user agent
    pub user_agent: Option<String>,
}

impl ClientConfig {
    /// Creates a new configuration with an API token.
    pub fn with_token(token: impl Into<String>) -> Self {
        Self {
            environment: Environment::default(),
            auth: AuthConfig::ApiToken(SecretString::from(token.into())),
            timeout: Duration::from_secs(30),
            user_agent: None,
        }
    }

    /// Sets the environment to production.
    pub fn production(mut self) -> Self {
        self.environment = Environment::Production;
        self
    }

    /// Sets the environment to sandbox.
    pub fn sandbox(mut self) -> Self {
        self.environment = Environment::Sandbox;
        self
    }

    /// Sets a custom timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets a custom user agent.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
}
