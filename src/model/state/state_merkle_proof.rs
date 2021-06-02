use crate::H256;

use super::MerkleTree;

/// StateMerkleProof
///
#[derive(Debug, Deserialize, Serialize)]
pub struct StateMerkleProof {
    pub state_hash: H256,
    pub merkle_tree: MerkleTree,
    pub root_hash: H256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf_value: Option<H256>,
    pub valid: bool,
}
