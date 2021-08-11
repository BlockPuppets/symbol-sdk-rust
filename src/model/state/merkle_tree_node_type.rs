/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
#[repr(u8)]
pub enum MerkleTreeNodeType {
    Branch = 0x00,
    Leaf = 0xff,
}
