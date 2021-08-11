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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionFees {
    /// Fee multiplier applied to transactions contained in block.
    #[serde(rename = "averageFeeMultiplier")]
    pub average_fee_multiplier: u64,
    /// Fee multiplier applied to transactions contained in block.
    #[serde(rename = "medianFeeMultiplier")]
    pub median_fee_multiplier: u64,
    /// Fee multiplier applied to transactions contained in block.
    #[serde(rename = "highestFeeMultiplier")]
    pub highest_fee_multiplier: u64,
    /// Fee multiplier applied to transactions contained in block.
    #[serde(rename = "lowestFeeMultiplier")]
    pub lowest_fee_multiplier: u64,
    /// Fee multiplier applied to transactions contained in block.
    #[serde(rename = "minFeeMultiplier")]
    pub min_fee_multiplier: u64,
}

impl fmt::Display for TransactionFees {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
