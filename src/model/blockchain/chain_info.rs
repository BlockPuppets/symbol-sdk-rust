use super::FinalizedBlock;
use std::fmt;

/// The chain information.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct ChainInfo {
    /// Current chain height.
    pub height: u64,

    /// Low part of the blockchain score.
    pub score_low: u64,

    /// High part of the blockchain score.
    pub score_high: u64,

    /// Latest finalized block.
    pub latest_finalized_block: FinalizedBlock,
}

impl fmt::Display for ChainInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}