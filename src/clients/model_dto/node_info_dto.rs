/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */
use std::convert::TryFrom;

use anyhow::Result;

use crate::account::PublicAccount;
use crate::network::NetworkType;
use crate::node::NodeInfo;

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeInfoDto {
    pub version: u32,
    pub public_key: String,
    pub network_generation_hash_seed: String,
    pub roles: u8,
    pub port: u16,
    pub network_identifier: u8,
    pub friendly_name: String,
    pub host: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_public_key: Option<String>,
}

impl NodeInfoDto {
    pub fn to_compact(&self) -> Result<NodeInfo> {
        let network_type = NetworkType::try_from(self.network_identifier)?;
        let mut node_public_key = None;

        if let Some(ref pk) = self.node_public_key {
            node_public_key = Some(pk.parse()?);
        }

        Ok(NodeInfo {
            version: self.version,
            public_account: PublicAccount::from_public_key(self.public_key.to_owned(), network_type)?,
            network_generation_hash_seed: self.network_generation_hash_seed.parse()?,
            roles: self.roles,
            port: self.port,
            network_type,
            friendly_name: self.friendly_name.to_owned(),
            host: self.host.to_owned(),
            node_public_key,
        })
    }
}


