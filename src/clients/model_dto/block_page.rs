use crate::blockchain::BlockInfo;
use crate::clients::model_dto::{BlockInfoDto, Pagination};
use crate::H192;
use anyhow::Result;

#[derive(Serialize, Deserialize)]
pub struct BlockPageDto {
    /// Array of blocks.
    pub data: Vec<BlockInfoDto>,
    pub pagination: Pagination,
}

impl BlockPageDto {
    pub fn to_compact(&self) -> Result<Vec<BlockInfo<H192>>> {
        let mut block_info_vec = vec![];

        for info in self.data.iter() {
            block_info_vec.push(info.to_compat()?)
        }

        Ok(block_info_vec)
    }
}
