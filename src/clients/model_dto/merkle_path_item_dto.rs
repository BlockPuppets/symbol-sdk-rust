use crate::blockchain::MerklePosition;

/// MerklePathItemDto : Each merkle path item is composed of a hash,
/// and a position relative to the proofHash being evaluated.
///
#[derive(Clone, Serialize, Deserialize)]
pub struct MerklePathItemDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<MerklePosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}