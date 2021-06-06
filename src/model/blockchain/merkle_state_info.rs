/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::state::MerkleTree;

/// The merkle path information clients can use to proof the state of the given entity.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct MerkleStateInfo {
    /// The hex information of the complete merkle tree as returned by server api.
    /// More information can be found in chapter 4.3 of the catapult whitepaper.
    pub raw: String,
    /// The merkle tree object parsed from raw.
    pub tree: MerkleTree,
}
