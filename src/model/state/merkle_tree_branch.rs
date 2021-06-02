use std::any::Any;

use crate::H256;

use super::{MerkleTreeBranchLink, MerkleTreeNodeType, MerkleTreeTrait};

/// Merkle tree branch node.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct MerkleTreeBranch {
    /// Merkle tree node type
    pub r#type: MerkleTreeNodeType,
    /// Branch node path.
    pub path: String,
    /// Branch node path encoded.
    pub encoded_path: String,
    /// Leaf nibble count.
    pub nibble_count: usize,
    /// Branch node link bitmask.
    pub link_mask: String,
    /// Branch node links.
    pub links: Vec<MerkleTreeBranchLink>,
    /// Branch node hash.
    pub branch_hash: H256,
}

impl MerkleTreeTrait for MerkleTreeBranch {
    fn get_type(&self) -> MerkleTreeNodeType {
        MerkleTreeNodeType::Branch
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
