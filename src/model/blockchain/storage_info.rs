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

/// The blockchain storage info structure describes stored data.
///
#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageInfo {
    /// The number of confirmed blocks.
    pub num_blocks: u64,

    /// The number of confirmed transactions.
    pub num_transactions: u64,

    /// The number accounts published in the blockchain.
    pub num_accounts: u64,
}

impl fmt::Display for StorageInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
