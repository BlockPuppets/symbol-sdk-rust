use super::MerklePathItem;
use std::fmt;

/// The block merkle proof info
///
#[derive(Debug, Deserialize, Serialize)]
pub struct MerkleProofInfo {
    /// Array of merkle path items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merkle_path: Option<Vec<MerklePathItem>>,
}

impl fmt::Display for MerkleProofInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}