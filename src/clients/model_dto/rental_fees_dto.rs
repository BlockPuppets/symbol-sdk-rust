/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::str::FromStr;

use anyhow::Result;

use crate::network::RentalFees;

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RentalFeesDto {
    /// Absolute amount. An amount of 123456789 (absolute) for a mosaic with divisibility 6 means 123.456789 (relative).
    pub effective_root_namespace_rental_fee_per_block: String,
    /// Absolute amount. An amount of 123456789 (absolute) for a mosaic with divisibility 6 means 123.456789 (relative).
    pub effective_child_namespace_rental_fee: String,
    /// Absolute amount. An amount of 123456789 (absolute) for a mosaic with divisibility 6 means 123.456789 (relative).
    pub effective_mosaic_rental_fee: String,
}

impl RentalFeesDto {
    pub fn to_compact(&self) -> Result<RentalFees> {
        Ok(RentalFees {
            effective_root_namespace_rental_fee_per_block: u64::from_str(
                &self.effective_root_namespace_rental_fee_per_block,
            )?,
            effective_child_namespace_rental_fee: u64::from_str(
                &self.effective_child_namespace_rental_fee,
            )?,
            effective_mosaic_rental_fee: u64::from_str(&self.effective_mosaic_rental_fee)?,
        })
    }
}
