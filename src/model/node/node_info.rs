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

use crate::account::PublicAccount;
use crate::H256;
use crate::network::NetworkType;

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Version of the application.
    pub version: u32,
    /// Public account.
    pub public_account: PublicAccount,
    pub network_generation_hash_seed: H256,
    /// A number that defines the different roles the node provides.
    /// Possible roles are: * 1 - Peer node.
    /// * 2 - Api node. * 4 - Voting node.
    /// * 64 - IPv4 compatible node * 128 - IPv6 compatible node.
    ///  The values are bitwise added together, Examples:
    /// 1 = Just Peer.
    /// 2 = Just Api.
    /// 3 = Peer and Api node.
    /// 7 = Peer, Api and Voting node.
    /// 65 = IPv4 and Peer node.
    pub roles: u8,
    /// Port used for the communication.
    pub port: u16,
    pub network_type: NetworkType,
    /// Node friendly name.
    pub friendly_name: String,
    /// Node IP address.
    pub host: String,
    /// Public key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_public_key: Option<H256>,
}

impl fmt::Display for NodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
