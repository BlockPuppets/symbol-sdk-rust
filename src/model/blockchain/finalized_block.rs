use crate::H256;

/// The finalized block.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct FinalizedBlock {
    /// Block height.
    pub height: u64,
    /// hash Block hash.
    pub hash: H256,
    /// Block finalization point.
    pub finalization_point: u64,
    /// Block finalization epoch.
    pub finalization_epoch: u64,
}
