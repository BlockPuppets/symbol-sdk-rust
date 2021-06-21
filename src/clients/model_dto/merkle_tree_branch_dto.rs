/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use anyhow::Result;

use crate::model_dto::MerkleTreeBranchLinkDto;
use crate::state::{MerkleTreeBranch, MerkleTreeNodeType};

/// MerkleTreeBranchDto : Merkle tree branch node.
///
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MerkleTreeBranchDto {
    #[serde(rename = "type")]
    pub _type: u8,
    /// Branch link path.
    pub path: String,
    /// Encoded branch link path.
    pub encoded_path: String,
    /// Nibble count.
    pub nibble_count: usize,
    /// Branch link bitmask.
    pub link_mask: String,
    /// Branch links (max 16).
    pub links: Vec<MerkleTreeBranchLinkDto>,
    pub branch_hash: String,
}

impl MerkleTreeBranchDto {
    pub fn to_compact(&self) -> Result<MerkleTreeBranch> {
        let mut links = vec![];
        for link in self.links.iter() {
            links.push(link.to_compact()?)
        }

        Ok(MerkleTreeBranch {
            r#type: MerkleTreeNodeType::Branch,
            path: self.path.to_owned(),
            encoded_path: self.encoded_path.to_owned(),
            nibble_count: self.nibble_count,
            link_mask: self.link_mask.to_owned(),
            links,
            branch_hash: self.branch_hash.parse()?,
        })
    }
}
