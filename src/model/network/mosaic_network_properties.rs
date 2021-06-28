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

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MosaicNetworkProperties {
    /// Maximum number of mosaics that an account can own.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mosaics_per_account: Option<String>,
    /// Maximum mosaic duration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mosaic_duration: Option<String>,
    /// Maximum mosaic divisibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mosaic_divisibility: Option<String>,
    /// Address encoded using a 32-character set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mosaic_rental_fee_sink_address: Option<String>,
    /// Mosaic rental fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mosaic_rental_fee: Option<String>,
}

impl fmt::Display for MosaicNetworkProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}