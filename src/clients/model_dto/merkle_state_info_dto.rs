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
use serde_json::Value;

use crate::blockchain::MerkleStateInfo;
use crate::model_dto::{MerkleTreeBranchDto, MerkleTreeLeafDto};
use crate::state::{MerkleTree, MerkleTreeBranch, MerkleTreeLeaf, MerkleTreeNodeType};

/// MerkleStateInfoDto : The merkle path information clients can use to proof the state of the given entity.
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MerkleStateInfoDto {
    /// The hex information of the complete merkle tree as returned by server api. More information can be found in chapter 4.3 of the catapult whitepaper.
    pub raw: String,
    /// Merkle tree parsed from merkle tree raw.
    pub tree: Vec<Value>,
}

impl MerkleStateInfoDto {
    pub fn to_compact(&self) -> Result<MerkleStateInfo> {
        if self.tree.is_empty() {
            return Ok(MerkleStateInfo {
                raw: self.raw.to_owned(),
                tree: MerkleTree::from_raw(&self.raw)?,
            });
        } else {
            let mut branches: Vec<MerkleTreeBranch> = vec![];

            let mut leaf: Option<MerkleTreeLeaf> = None;

            for tree in self.clone().tree.into_iter() {
                if tree["type"].as_u64().unwrap() as u8 == MerkleTreeNodeType::Branch as u8 {
                    let branch: MerkleTreeBranchDto = serde_json::from_value(tree)?;
                    branches.push(branch.to_compact()?);
                } else {
                    let leaf_dto: MerkleTreeLeafDto = serde_json::from_value(tree)?;
                    leaf = Some(leaf_dto.to_compact()?);
                }
            }

            Ok(MerkleStateInfo {
                raw: self.raw.to_owned(),
                tree: MerkleTree { branches, leaf },
            })
        }
    }
}
