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

use crate::state::MerkleTreeBranchLink;

/// MerkleTreeBranchLinkDto : Merkle tree branch link.
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MerkleTreeBranchLinkDto {
    /// Branch link nibble bit index (hexadecimal).
    #[serde(rename = "bit")]
    pub bit: String,
    #[serde(rename = "link")]
    pub link: String,
}

impl MerkleTreeBranchLinkDto {
    /// Merkle tree branch link.
    pub fn to_compact(&self) -> Result<MerkleTreeBranchLink> {
        Ok(MerkleTreeBranchLink {
            bit: self.bit.to_owned(),
            link: self.link.parse()?,
        })
    }
}