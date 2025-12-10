//! CLI configuration handling.

use serde::Deserialize;
use std::path::Path;

/// CLI configuration.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// API token
    pub api_token: String,
}

/// Load configuration from file or environment.
pub fn load_config(path: Option<&Path>) -> anyhow::Result<Config> {
    // Try config file first
    if let Some(path) = path {
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            return Ok(toml::from_str(&content)?);
        }
    }

    // Try default config location
    if let Some(config_dir) = directories::ProjectDirs::from("com", "paraplu", "wise") {
        let config_file = config_dir.config_dir().join("config.toml");
        if config_file.exists() {
            let content = std::fs::read_to_string(&config_file)?;
            return Ok(toml::from_str(&content)?);
        }
    }

    // Try environment variable
    if let Ok(token) = std::env::var("WISE_API_TOKEN") {
        return Ok(Config { api_token: token });
    }

    anyhow::bail!(
        "No configuration found. Set WISE_API_TOKEN environment variable or create a config file."
    )
}
