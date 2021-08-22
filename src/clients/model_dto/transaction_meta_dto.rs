/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize)]
pub struct TransactionMetaDto {
    /// Height of the blockchain.
    pub height: String,
    pub hash: String,
    pub merkle_component_hash: String,
    /// Transaction index within the block.
    pub index: i32,
}


