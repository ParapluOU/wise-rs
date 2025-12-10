//! Flexible datetime deserialization for Wise API.
//!
//! The Wise API returns dates in various formats:
//! - "2025-09-07T11:29:17" (ISO format without timezone)
//! - "2025-09-07 11:29:17" (space instead of T)
//! - "2025-09-07T11:29:17Z" (with timezone)

use chrono::NaiveDateTime;
use serde::{self, Deserialize, Deserializer};

/// Deserialize a datetime that may have various formats.
pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_datetime(&s).map_err(serde::de::Error::custom)
}

/// Deserialize an optional datetime.
pub fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) => parse_datetime(&s).map(Some).map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

fn parse_datetime(s: &str) -> Result<NaiveDateTime, String> {
    // Try various formats
    let formats = [
        "%Y-%m-%dT%H:%M:%S%.fZ",    // ISO with fractional seconds and Z
        "%Y-%m-%dT%H:%M:%SZ",       // ISO with Z
        "%Y-%m-%dT%H:%M:%S%.f",     // ISO with fractional seconds
        "%Y-%m-%dT%H:%M:%S",        // ISO basic
        "%Y-%m-%d %H:%M:%S%.f",     // Space separator with fractional
        "%Y-%m-%d %H:%M:%S",        // Space separator
    ];

    for fmt in &formats {
        if let Ok(dt) = NaiveDateTime::parse_from_str(s, fmt) {
            return Ok(dt);
        }
    }

    // Try stripping timezone offset if present (e.g., +00:00)
    if let Some(pos) = s.rfind('+').or_else(|| s.rfind('-').filter(|&p| p > 10)) {
        let without_tz = &s[..pos];
        for fmt in &formats[2..] {
            // Skip the Z formats
            if let Ok(dt) = NaiveDateTime::parse_from_str(without_tz, fmt) {
                return Ok(dt);
            }
        }
    }

    Err(format!("Unable to parse datetime: {}", s))
}
