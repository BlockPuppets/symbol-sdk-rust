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
use std::fmt::Debug;

use anyhow::Result;

use super::{MerkleTreeBranch, MerkleTreeLeaf, MerkleTreeNodeType, MerkleTreeParser};

/// Merkle tree.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct MerkleTree {
    /// The branches.
    pub branches: Vec<MerkleTreeBranch>,
    /// The leaf the leaf.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf: Option<MerkleTreeLeaf>,
}

impl MerkleTree {
    pub fn from_raw(raw: &str) -> Result<MerkleTree> {
        let merkle_tree = MerkleTreeParser::parse_merkle_tree_from_raw(&hex::decode(raw)?)?;

        let mut branches: Vec<MerkleTreeBranch> = vec![];

        let mut leaf: Option<MerkleTreeLeaf> = None;

        for tree in merkle_tree.into_iter() {
            if tree.get_type() == MerkleTreeNodeType::Branch {
                branches.push(*tree.into_any().downcast::<MerkleTreeBranch>().unwrap());
            } else {
                leaf = Some(*tree.into_any().downcast::<MerkleTreeLeaf>().unwrap())
            }
        }

        Ok(Self {
            branches,
            leaf,
        })
    }
}

pub trait MerkleTreeTrait: Debug {
    fn get_type(&self) -> MerkleTreeNodeType;
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}
