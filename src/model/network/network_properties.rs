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

use crate::NodeIdentityEqualityStrategy;

/// NetworkPropertiesDto : Network related configuration properties.
///
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkProperties {
    /// Network identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_equality_strategy: Option<NodeIdentityEqualityStrategy>,
    /// Public key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nemesis_signer_public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_hash_seed: Option<String>,
    /// Nemesis epoch time adjustment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch_adjustment: Option<String>,
}

impl fmt::Display for NetworkProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
