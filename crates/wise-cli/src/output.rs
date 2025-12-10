//! Output formatting for CLI.

use serde::Serialize;

/// Output format for CLI commands.
#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum OutputFormat {
    /// Human-readable output
    #[default]
    Human,
    /// JSON output
    Json,
}

/// Print output in the specified format.
pub fn print<T: Serialize + std::fmt::Debug>(value: &T, format: OutputFormat) {
    match format {
        OutputFormat::Human => println!("{:#?}", value),
        OutputFormat::Json => {
            if let Ok(json) = serde_json::to_string_pretty(value) {
                println!("{}", json);
            }
        }
    }
}
