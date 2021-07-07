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

use crate::model_dto::{MosaicInfoDto, Pagination};
use crate::mosaic::MosaicInfo;

#[derive(Serialize, Deserialize)]
pub struct MosaicPageDto {
    /// Array of blocks.
    pub data: Vec<MosaicInfoDto>,
    pub pagination: Pagination,
}

impl MosaicPageDto {
    pub fn to_compact(&self) -> Result<Vec<MosaicInfo>> {
        let mut block_info_vec = vec![];

        for info in self.data.iter() {
            block_info_vec.push(info.to_compact()?)
        }

        Ok(block_info_vec)
    }
}
