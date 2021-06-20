/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::account::Address;
use crate::ParamSearchCriteria;

/// Defines the params used to search mosaics. With this criteria, you can sort and filter
/// mosaics queries using rest.
///
#[serde(rename_all = "camelCase")]
#[derive(Clone, Serialize, Deserialize)]
pub struct MosaicSearchCriteria {
    /// Filter by owner address.
    pub owner_address: Option<Address>,

    pub param: Option<ParamSearchCriteria>,
}
