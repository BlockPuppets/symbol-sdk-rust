/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::{GenerationHash, H256, H512};
use crate::account::{Address, PublicAccount};
use crate::network::NetworkType;
use crate::ser_to_hex_upper;

use super::BlockType;

/// The block info structure describes basic information of a new gernated block (Websocket payload).
///
#[derive(Debug, Deserialize, Serialize)]
pub struct NewBlockInfo {
    /// The block hash.
    pub hash: H256,

    /// The generation hash
    pub generation_hash: GenerationHash,

    /// The fee_multiplier defined by the harvester.
    pub fee_multiplier: usize,

    /// The block signature.
    /// The signature was generated by the signer and can be used to validate that the blockchain
    /// data was not modified by a node.
    #[serde(serialize_with = "ser_to_hex_upper")]
    pub signature: H512,

    /// The public account of block harvester.
    pub signer: PublicAccount,

    /// The network type.
    pub network_type: NetworkType,

    /// The transaction version.
    pub version: u8,

    /// The block type.
    pub r#type: BlockType,

    /// The height of which the block was confirmed.
    /// Each block has a unique height. Subsequent blocks differ in height by 1.
    pub height: u64,

    /// The number of milliseconds elapsed since the creation of the nemesis blockchain.
    pub timestamp: u64,

    /// The POI difficulty to harvest a block.
    pub difficulty: u64,

    /// The last block hash.
    pub previous_block_hash: H256,

    /// The block transaction hash.
    pub block_transactions_hash: H256,

    /// The block receipt hash.
    pub block_receipts_hash: H256,

    /// The state hash.
    pub state_hash: String,

    /// The proof gamma.
    pub proof_gamma: H256,

    /// The proof scalar.
    pub proof_scalar: H256,

    /// The proof verification hash.
    pub proof_verification_hash: String,

    /// The beneficiary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_address: Option<Address>,
}
