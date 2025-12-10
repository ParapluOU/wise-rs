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
    // Try to load .secrets.env from current directory or parent directories
    load_secrets_env();

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

    // Try environment variables (WISE_API_KEY_RO takes precedence)
    if let Ok(token) = std::env::var("WISE_API_KEY_RO") {
        return Ok(Config { api_token: token });
    }
    if let Ok(token) = std::env::var("WISE_API_TOKEN") {
        return Ok(Config { api_token: token });
    }

    anyhow::bail!(
        "No configuration found. Set WISE_API_KEY_RO environment variable or create a config file."
    )
}

/// Load .secrets.env file from current directory or parent directories.
fn load_secrets_env() {
    let mut dir = std::env::current_dir().ok();
    while let Some(current) = dir {
        let secrets_file = current.join(".secrets.env");
        if secrets_file.exists() {
            // dotenvy::from_path_override loads and sets env vars
            let _ = dotenvy::from_path(&secrets_file);
            break;
        }
        dir = current.parent().map(|p| p.to_path_buf());
    }
}
