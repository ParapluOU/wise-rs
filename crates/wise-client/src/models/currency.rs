//! Currency models.

use serde::{Deserialize, Serialize};

/// Supported currency.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    /// Currency code (e.g., "USD", "EUR")
    pub code: String,
    /// Currency name
    pub name: String,
    /// Currency symbol
    pub symbol: String,
    /// Decimal places
    pub decimal_digits: Option<u8>,
}
