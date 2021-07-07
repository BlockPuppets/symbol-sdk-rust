/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::any::Any;

use crate::H256;

use super::{MerkleTreeBranchLink, MerkleTreeNodeType, MerkleTreeTrait};

/// Merkle tree branch node.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct MerkleTreeBranch {
    /// Merkle tree node type
    pub r#type: MerkleTreeNodeType,
    /// Branch node path.
    pub path: String,
    /// Branch node path encoded.
    pub encoded_path: String,
    /// Leaf nibble count.
    pub nibble_count: usize,
    /// Branch node link bitmask.
    pub link_mask: String,
    /// Branch node links.
    pub links: Vec<MerkleTreeBranchLink>,
    /// Branch node hash.
    pub branch_hash: H256,
}

impl MerkleTreeTrait for MerkleTreeBranch {
    fn get_type(&self) -> MerkleTreeNodeType {
        MerkleTreeNodeType::Branch
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
