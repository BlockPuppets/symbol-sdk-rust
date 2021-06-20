/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::any::Any;

use crate::H256;

use super::{MerkleTreeNodeType, MerkleTreeTrait};

/// Merkle tree leaf node.
///
#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Serialize)]
pub struct MerkleTreeLeaf {
    /// Merkle tree node type.
    pub r#type: MerkleTreeNodeType,
    /// Leaf node path.
    pub path: String,
    /// Leaf node path encoded.
    pub encoded_path: String,
    /// Leaf nibble count.
    pub nibble_count: usize,
    /// Leaf node value hash.
    pub value: H256,
    /// Leaf node hash.
    pub leaf_hash: H256,
}

impl MerkleTreeTrait for MerkleTreeLeaf {
    fn get_type(&self) -> MerkleTreeNodeType {
        MerkleTreeNodeType::Leaf
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
