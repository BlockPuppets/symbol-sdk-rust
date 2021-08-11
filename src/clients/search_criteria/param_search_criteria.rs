/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::Order;

/// Defines the params used to search mosaics. With this criteria, you can sort and filter
/// mosaics queries using rest.
///
#[serde(rename_all = "camelCase")]
#[derive(Clone, Serialize, Deserialize)]
pub struct ParamSearchCriteria {
    /// Select the number of entries to return.
    /// * Default: 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u8>,

    /// Filter by page number.
    /// * Default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<u8>,

    /// Entry id at which to start pagination.
    /// If the ordering parameter is set to -id, the elements returned precede the identifier.
    /// Otherwise, newer elements with respect to the id are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,

    /// Sort responses in ascending or descending order based on the collection property set on the param orderBy.
    /// If the request does not specify orderBy, REST returns the collection ordered by id.
    /// * Default: "desc"
    /// * Enum: "asc" "desc"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}
