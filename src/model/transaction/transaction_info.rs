/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::transaction::common_transaction::Height;
use crate::H256;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInfo {
    /// The block height in which the transaction was included.
    pub height: Height,
    /// The index representing either transaction index/position within block or within an aggregate transaction.
    pub index: u32,
    /// The transaction db id.
    pub id: String,
    /// The transaction hash.
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<H256>,
    /// The transaction merkle hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merkle_component_hash: Option<H256>,
}

impl TransactionInfo {
    pub fn transaction_hash(&self) -> H256 {
        match self.hash.to_owned() {
            Some(h) => h,
            None => H256::default(),
        }
    }
}

impl core::fmt::Display for TransactionInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
