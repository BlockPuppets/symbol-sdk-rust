/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::str::FromStr;

use anyhow::Result;

use crate::blockchain::FinalizedBlock;
use crate::H256;

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
        Ok(FinalizedBlock {
            height: self.height.parse::<u64>()?,
            hash: H256::from_str(self.hash.as_str())?,
            finalization_point: self.finalization_point,
            finalization_epoch: self.finalization_epoch,
        })
    }
}