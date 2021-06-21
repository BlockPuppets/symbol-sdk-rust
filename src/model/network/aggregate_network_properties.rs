/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
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
pub struct AggregateNetworkProperties {
    /// Maximum number of transactions per aggregate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_transactions_per_aggregate: Option<String>,
    /// Maximum number of cosignatures per aggregate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cosignatures_per_aggregate: Option<String>,
    /// Set to true if cosignatures must exactly match component signers. Set to false if cosignatures should be validated externally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_strict_cosignature_check: Option<bool>,
    /// Set to true if bonded aggregates should be allowed. Set to false if bonded aggregates should be rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_bonded_aggregate_support: Option<bool>,
    /// Maximum lifetime a bonded transaction can have before it expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bonded_transaction_lifetime: Option<String>,
}

impl fmt::Display for AggregateNetworkProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}