/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::H256;

use super::MerkleTree;

/// StateMerkleProof
///
#[derive(Debug, Deserialize, Serialize)]
pub struct StateMerkleProof {
    pub state_hash: H256,
    pub merkle_tree: MerkleTree,
    pub root_hash: H256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf_value: Option<H256>,
    pub valid: bool,
}
