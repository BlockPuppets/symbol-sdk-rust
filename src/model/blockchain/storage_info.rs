/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

/// The blockchain storage info structure describes stored data.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct StorageInfo {
    /// The number of confirmed blocks.
    pub num_blocks: usize,

    /// The number of confirmed transactions.
    pub num_transactions: usize,

    /// The number accounts published in the blockchain.
    pub num_accounts: usize,
}
