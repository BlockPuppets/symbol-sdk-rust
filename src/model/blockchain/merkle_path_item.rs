use crate::H256;

use super::MerklePosition;
use std::fmt;

/// The block merkle path item
///
#[derive(Debug, Deserialize, Serialize)]
pub struct MerklePathItem {
    /// The position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<MerklePosition>,

    /// The hash
    pub hash: H256,
}

impl fmt::Display for MerklePathItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}