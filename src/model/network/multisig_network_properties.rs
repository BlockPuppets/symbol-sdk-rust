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
pub struct MultisigNetworkProperties {
    /// Maximum number of multisig levels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_multisig_depth: Option<String>,
    /// Maximum number of cosignatories per account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cosignatories_per_account: Option<String>,
    /// Maximum number of accounts a single account can cosign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cosigned_accounts_per_account: Option<String>,
}

impl fmt::Display for MultisigNetworkProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}