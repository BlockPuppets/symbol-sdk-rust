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

use super::{
    AccountKeyLinkNetworkProperties, AccountRestrictionNetworkProperties,
    AggregateNetworkProperties, HashLockNetworkProperties, MetadataNetworkProperties,
    MosaicNetworkProperties, MosaicRestrictionNetworkProperties, MultisigNetworkProperties,
    NamespaceNetworkProperties, SecretLockNetworkProperties, TransferNetworkProperties,
};

/// PluginsPropertiesDto : Plugin related configuration properties.
///
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginsProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_link: Option<AccountKeyLinkNetworkProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<AggregateNetworkProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_hash: Option<HashLockNetworkProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_secret: Option<SecretLockNetworkProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataNetworkProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mosaic: Option<MosaicNetworkProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multisig: Option<MultisigNetworkProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<NamespaceNetworkProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restriction_account: Option<AccountRestrictionNetworkProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restriction_mosaic: Option<MosaicRestrictionNetworkProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<TransferNetworkProperties>,
}

impl fmt::Display for PluginsProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
