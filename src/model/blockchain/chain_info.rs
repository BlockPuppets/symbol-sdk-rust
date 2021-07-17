/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::fmt;

use super::FinalizedBlock;

/// The chain information.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct ChainInfo {
    /// Current chain height.
    pub height: u64,

    /// Low part of the blockchain score.
    pub score_low: u64,

    /// High part of the blockchain score.
    pub score_high: u64,

    /// Latest finalized block.
    pub latest_finalized_block: FinalizedBlock,
}

impl fmt::Display for ChainInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
