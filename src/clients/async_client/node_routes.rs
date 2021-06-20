/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::{Client, Error, Response, RetryStrategy};
use crate::account::PublicAccount;
use crate::blockchain::StorageInfo;
use crate::clients::request::Request;
use crate::model_dto::{
    NodeHealthInfoDto, NodeInfoDto, NodeTimeDto, ServerInfoDto, UnlockedAccountDto,
};
use crate::node::{NodeHealth, NodeInfo, NodeTime, ServerInfo};

pub struct NodeApi<R: RetryStrategy>(pub(crate) Client<R>);

impl<R: RetryStrategy> NodeApi<R> {
    /// Get the node health information.
    ///
    /// # Info
    /// Supplies information regarding the connection and services status.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an `NodeHealth` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_node_health(&self) -> Result<NodeHealth, Error> {
        let resp: Response<NodeHealthInfoDto> =
            self.as_ref().send(Request::get_node_health()).await?;
        Ok(resp.status.clone())
    }

    /// Get the node information.
    ///
    /// # Info
    /// Supplies additional information about the application running on a node.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an `NodeInfo` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_node_info(&self) -> Result<NodeInfo, Error> {
        let resp: Response<NodeInfoDto> = self.as_ref().send(Request::get_node_info()).await?;
        resp.to_compact()
            .map_err(|e| Error::unexpected_uncategorized(e.to_string()))
    }

    /// Get peers information.
    ///
    /// # Info
    /// Gets the list of peers visible by the node.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an vec of `NodeInfo` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_node_peers(&self) -> Result<Vec<NodeInfo>, Error> {
        let resp: Response<Vec<NodeInfoDto>> =
            self.as_ref().send(Request::get_node_peers()).await?;
        let mut nodes = vec![];

        for node in &*resp {
            nodes.push(
                node.to_compact()
                    .map_err(|e| Error::unexpected_uncategorized(e.to_string()))?,
            )
        }
        Ok(nodes)
    }

    /// Get the storage information of the node.
    ///
    /// # Info
    /// Gets the node time at the moment the reply was sent and received.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an `StorageInfo` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_storage_info(&self) -> Result<StorageInfo, Error> {
        let resp: Response<StorageInfo> = self.as_ref().send(Request::get_storage_info()).await?;
        Ok((*resp).clone())
    }

    /// Get the node time.
    ///
    /// # Returns
    /// Returns storage information about the node.
    ///
    /// A `Result` whose okay value is an `StorageInfo` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_node_time(&self) -> Result<NodeTime, Error> {
        let resp: Response<NodeTimeDto> = self.as_ref().send(Request::get_node_time()).await?;
        resp.to_compact()
            .map_err(|e| Error::unexpected_uncategorized(e.to_string()))
    }

    /// Get the version of the running REST component.
    ///
    /// # Returns
    /// Returns the version of the running catapult-rest component.
    ///
    /// A `Result` whose okay value is an `ServerInfo` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_server_info(&self) -> Result<ServerInfo, Error> {
        let resp: Response<ServerInfoDto> = self.as_ref().send(Request::get_server_info()).await?;
        Ok(resp.server_info.clone())
    }

    /// Get the unlocked harvesting account `PublicAccount`.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an vec of `PublicAccount` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_unlocked_accounts(&self) -> Result<Vec<PublicAccount>, Error> {
        let resp: Response<UnlockedAccountDto> =
            self.as_ref().send(Request::get_unlocked_accounts()).await?;
        Ok(resp
            .unlocked_account
            .iter()
            .map(|public_key| {
                PublicAccount::from_public_key(public_key, self.as_ref().network_type).unwrap()
            })
            .collect())
    }
}

impl<R: RetryStrategy> AsRef<Client<R>> for NodeApi<R> {
    fn as_ref(&self) -> &Client<R> {
        &self.0
    }
}
