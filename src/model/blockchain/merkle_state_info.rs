/*
 * // Copyright 2021 BlockPuppets developers.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::fmt;

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

impl fmt::Display for MerkleStateInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}