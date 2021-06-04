use crate::blockchain::{MerklePathItem, MerklePosition};
use crate::H256;
use anyhow::Result;
use std::str::FromStr;

/// MerklePathItemDto : Each merkle path item is composed of a hash,
/// and a position relative to the proofHash being evaluated.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerklePathItemDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<MerklePosition>,
    pub hash: String,
}

impl MerklePathItemDto {
    pub fn to_compact(&self) -> Result<MerklePathItem> {
        Ok(MerklePathItem {
            position: self.position.clone(),
            hash: H256::from_str(self.hash.as_str())?,
        })
    }
}
