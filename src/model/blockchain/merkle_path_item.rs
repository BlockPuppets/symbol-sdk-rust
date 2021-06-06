/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::fmt;

use crate::H256;

use super::MerklePosition;

/// The block merkle path item
///
#[derive(Debug, Deserialize, Serialize)]
pub struct MerklePathItem {
    /// The position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<MerklePosition>,

    /// The hash
    pub hash: H256,
}

impl fmt::Display for MerklePathItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}