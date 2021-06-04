use crate::H256;
use std::fmt;

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

impl fmt::Display for FinalizedBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}