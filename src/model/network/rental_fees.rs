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

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RentalFees {
    /// Absolute amount. An amount of 123456789 (absolute) for a mosaic with divisibility 6 means 123.456789 (relative).
    pub effective_root_namespace_rental_fee_per_block: u64,
    /// Absolute amount. An amount of 123456789 (absolute) for a mosaic with divisibility 6 means 123.456789 (relative).
    pub effective_child_namespace_rental_fee: u64,
    /// Absolute amount. An amount of 123456789 (absolute) for a mosaic with divisibility 6 means 123.456789 (relative).
    pub effective_mosaic_rental_fee: u64,
}

impl fmt::Display for RentalFees {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
