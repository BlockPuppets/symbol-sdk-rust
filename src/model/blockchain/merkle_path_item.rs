use crate::H256;

use super::MerklePosition;

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
