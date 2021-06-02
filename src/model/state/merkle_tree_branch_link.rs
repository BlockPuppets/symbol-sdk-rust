use crate::H256;

/// Merkle tree branch link.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct MerkleTreeBranchLink {
    /// Link bit index
    pub bit: String,
    /// Link hash
    pub link: H256,
}