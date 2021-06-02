use super::FinalizedBlock;

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
