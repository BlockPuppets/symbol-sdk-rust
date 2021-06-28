/*
 * // Copyright 2021 BlockPuppets developers.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use anyhow::Result;

use crate::state::{MerkleTreeLeaf, MerkleTreeNodeType};

/// MerkleTreeLeafDto : Merkle tree leaf node.
///
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MerkleTreeLeafDto {
    #[serde(rename = "type")]
    pub _type: u8,
    /// Leaf path.
    pub path: String,
    /// Encoded leaf path.
    pub encoded_path: String,
    /// Nibble count.
    pub nibble_count: usize,
    /// Leaf value (sha256 hash).
    pub value: String,
    pub leaf_hash: String,
}

impl MerkleTreeLeafDto {
    pub fn to_compact(&self) -> Result<MerkleTreeLeaf> {
        Ok(MerkleTreeLeaf {
            r#type: MerkleTreeNodeType::Leaf,
            path: self.path.to_owned(),
            encoded_path: self.encoded_path.to_owned(),
            nibble_count: self.nibble_count,
            value: self.value.parse()?,
            leaf_hash: self.leaf_hash.parse()?,
        })
    }
}