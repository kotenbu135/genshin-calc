use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum GoodError {
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("invalid GOOD format: expected \"GOOD\", got \"{0}\"")]
    InvalidFormat(String),

    #[error("unsupported GOOD version: {0}")]
    UnsupportedVersion(u8),
}

#[derive(Debug, Clone, Serialize)]
pub enum ImportWarning {
    UnknownCharacter(String),
    UnknownWeapon(String),
    UnknownArtifactSet(String),
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
