/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use anyhow::Result;

use crate::blockchain::BlockInfo;
use crate::clients::model_dto::{BlockInfoDto, Pagination};

#[derive(Serialize, Deserialize)]
pub struct BlockPageDto {
    /// Array of blocks.
    pub data: Vec<BlockInfoDto>,
    pub pagination: Pagination,
}

impl BlockPageDto {
    pub fn to_compact(&self) -> Result<Vec<BlockInfo>> {
        let mut block_info_vec = vec![];

        for info in self.data.iter() {
            block_info_vec.push(info.to_compact()?)
        }

        Ok(block_info_vec)
    }
}
