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
pub struct NamespaceNetworkProperties {
    /// Maximum namespace name size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_name_size: Option<String>,
    /// Maximum number of children for a root namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_child_namespaces: Option<String>,
    /// Maximum namespace depth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_namespace_depth: Option<String>,
    /// Minimum namespace duration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_namespace_duration: Option<String>,
    /// Maximum namespace duration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_namespace_duration: Option<String>,
    /// Grace period during which time only the previous owner can renew an expired namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_grace_period_duration: Option<String>,
    /// Reserved root namespaces that cannot be claimed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_root_namespace_names: Option<String>,
    /// Address encoded using a 32-character set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_rental_fee_sink_address: Option<String>,
    /// Root namespace rental fee per block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_namespace_rental_fee_per_block: Option<String>,
    /// Child namespace rental fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_namespace_rental_fee: Option<String>,
}

impl fmt::Display for NamespaceNetworkProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
