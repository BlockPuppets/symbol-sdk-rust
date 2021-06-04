use crate::blockchain::FinalizedBlock;
use crate::H256;
use std::str::FromStr;
use anyhow::Result;

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FinalizedBlockDto {
    /// Finalization Epoch
    pub finalization_epoch: u64,
    /// Finalization point
    pub finalization_point: u64,
    /// Height of the blockchain.
    pub height: String,
    pub hash: String,
}

impl FinalizedBlockDto {
    pub fn to_compact(&self) -> Result<FinalizedBlock> {
        Ok(FinalizedBlock{
            height: self.height.parse::<u64>()?,
            hash: H256::from_str(self.hash.as_str())?,
            finalization_point: self.finalization_point,
            finalization_epoch: self.finalization_epoch
        })
    }
}