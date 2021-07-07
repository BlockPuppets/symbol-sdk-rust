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

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecretLockNetworkProperties {
    /// Maximum number of blocks for which a secret lock can exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_secret_lock_duration: Option<String>,
    /// Minimum size of a proof in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_proof_size: Option<String>,
    /// Maximum size of a proof in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_proof_size: Option<String>,
}

impl fmt::Display for SecretLockNetworkProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}