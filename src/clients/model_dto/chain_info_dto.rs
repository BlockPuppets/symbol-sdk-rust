use crate::clients::model_dto::FinalizedBlockDto;
use crate::blockchain::ChainInfo;
use anyhow::Result;

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainInfoDto {
    /// Height of the blockchain.
    pub height: String,
    /// Score of the blockchain. During synchronization, nodes try to get the blockchain with highest score in the network. 
    pub score_high: String,
    /// Score of the blockchain. During synchronization, nodes try to get the blockchain with highest score in the network. 
    pub score_low: String,
    pub latest_finalized_block: FinalizedBlockDto,
}

impl ChainInfoDto {
   pub fn to_compact(&self) -> Result<ChainInfo> {
        let latest_finalized_block = self.latest_finalized_block.to_compact()?;
        Ok(ChainInfo{
            height: self.height.parse::<u64>()?,
            score_low: self.score_low.parse::<u64>()?,
            score_high: self.score_high.parse::<u64>()?,
            latest_finalized_block
        })
    }
}