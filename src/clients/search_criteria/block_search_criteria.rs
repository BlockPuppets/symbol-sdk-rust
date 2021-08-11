/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::account::{Address, PublicAccount};
use crate::ParamSearchCriteria;

use super::block_order_by::BlockOrderBy;

/// Defines the params used to search blocks. With this criteria, you can sort and filter
/// block queries using rest.
///
#[serde(rename_all = "camelCase")]
#[derive(Clone, Serialize, Deserialize)]
pub struct BlockSearchCriteria {
    /// `PublicAccount` of the account signing the entity.
    /// Filter by `PublicAccount` of the account signing the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_public_key: Option<PublicAccount>,

    /// beneficiary `Address`.
    /// Filter by beneficiary Address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_address: Option<Address>,

    /// Order by block id or height.
    /// Sort responses by the property set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<BlockOrderBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<ParamSearchCriteria>,
}
