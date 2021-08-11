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

use crate::blockchain::MerkleProofInfo;

use super::MerklePathItemDto;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProofInfoDto {
    /// List of complementary merkle path items needed to recalculate the merkle root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merkle_path: Option<Vec<MerklePathItemDto>>,
}

impl MerkleProofInfoDto {
    pub fn to_compact(&self) -> Result<MerkleProofInfo> {
        let merkle_path = if let Some(ref item) = self.merkle_path {
            let mut merkle_path_vec = vec![];
            for item in item.into_iter() {
                merkle_path_vec.push(item.to_compact()?)
            }
            Some(merkle_path_vec)
        } else {
            None
        };

        Ok(MerkleProofInfo { merkle_path })
    }
}
