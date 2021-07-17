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
pub struct BlockMetaDto {
    pub hash: String,
    /// Absolute amount. An amount of 123456789 (absolute) for a mosaic with divisibility 6 means 123.456789 (relative).
    pub total_fee: String,
    pub generation_hash: String,
    pub state_hash_sub_cache_merkle_roots: Vec<String>,
    /// Total number of [transactions](https://docs.symbolplatform.com/concepts/transaction.html) confirmed in this block,
    /// including *embedded* transactions (i.e. transactions contained within aggregate transactions).
    pub total_transactions_count: u16,
    /// Number of [transactions](https://docs.symbolplatform.com/concepts/transaction.html) confirmed in this block.
    /// This does not count *embedded* transactions (i.e. transactions contained within aggregate transactions).
    pub transactions_count: u16,
    /// Number of statements (of any kind) present in this block. Bear in mind that some of them
    /// (like [resolution statements](https://docs.symbolplatform.com/concepts/receipt.html#resolution-statement))
    /// are triggered by transactions present in the block, but in general,
    /// [transaction statements](https://docs.symbolplatform.com/concepts/receipt.html#transaction-statement) are not.
    pub statements_count: u16,
}
