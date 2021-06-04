use std::fmt;

/// Position relative to the proofHash being evaluated.
///
#[derive(Debug, Clone, Deserialize, Serialize)]
#[repr(u8)]
pub enum MerklePosition {
    Left,
    Right,
}

impl fmt::Display for MerklePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}