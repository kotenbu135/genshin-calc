//! Error types for GOOD format import.

use serde::Serialize;

/// Errors that can occur during GOOD format import.
#[derive(Debug, thiserror::Error)]
pub enum GoodError {
    /// Failed to parse JSON string.
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    /// The `format` field in JSON is not "GOOD".
    #[error("invalid GOOD format: expected \"GOOD\", got \"{0}\"")]
    InvalidFormat(String),

    /// GOOD version is not supported (only v1-v3 supported).
    #[error("unsupported GOOD version: {0}")]
    UnsupportedVersion(u8),

    /// Character has no weapon equipped.
    /// This is required for building TeamMember stats.
    #[error("weapon is required for stat calculation")]
    MissingWeapon,
}

/// Warnings generated during import.
///
/// These are non-fatal - the import succeeds but some data couldn't be processed.
/// Use these warnings to inform users about missing or unknown data.
#[derive(Debug, Clone, Serialize)]
pub enum ImportWarning {
    /// Character key wasn't found in genshin-calc-data.
    /// Check if the character ID is correct.
    UnknownCharacter(String),

    /// Weapon key wasn't found in genshin-calc-data.
    /// Check if the weapon ID is correct.
    UnknownWeapon(String),

    /// Artifact set key wasn't found in genshin-calc-data.
    /// Check if the set name is correct.
    UnknownArtifactSet(String),

    /// A stat key wasn't recognized.
    /// This may be a new stat added in a game update.
    UnknownStatKey(String),
}

pub(crate) fn validate_format(good: &crate::types::GoodFormat) -> Result<(), GoodError> {
    if good.format != "GOOD" {
        return Err(GoodError::InvalidFormat(good.format.clone()));
    }
    if good.version == 0 || good.version > 3 {
        return Err(GoodError::UnsupportedVersion(good.version));
    }
    Ok(())
}
