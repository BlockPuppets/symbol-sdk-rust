/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::str::FromStr;

use anyhow::Result;

use crate::blockchain::{MerklePathItem, MerklePosition};
use crate::H256;

/// MerklePathItemDto : Each merkle path item is composed of a hash,
/// and a position relative to the proofHash being evaluated.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerklePathItemDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<MerklePosition>,
    pub hash: String,
}

impl MerklePathItemDto {
    pub fn to_compact(&self) -> Result<MerklePathItem> {
        Ok(MerklePathItem {
            position: self.position.clone(),
            hash: H256::from_str(self.hash.as_str())?,
        })
    }
}
