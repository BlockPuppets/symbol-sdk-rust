use crate::state::MerkleTree;

/// The merkle path information clients can use to proof the state of the given entity.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct MerkleStateInfo {
    /// The hex information of the complete merkle tree as returned by server api.
    /// More information can be found in chapter 4.3 of the catapult whitepaper.
    pub raw: String,
    /// The merkle tree object parsed from raw.
    pub tree: MerkleTree,
}
