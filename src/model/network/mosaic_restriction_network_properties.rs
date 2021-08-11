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
pub struct MosaicRestrictionNetworkProperties {
    /// Maximum number of mosaic restriction values.
    #[serde(
        rename = "maxMosaicRestrictionValues",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_mosaic_restriction_values: Option<String>,
}

impl fmt::Display for MosaicRestrictionNetworkProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
