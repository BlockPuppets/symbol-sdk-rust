/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::clients::request::Request;
use crate::network::{NetworkConfiguration, NetworkName};
use crate::{Client, Error, Response, RetryStrategy};

pub struct NetworkApi<R: RetryStrategy>(pub(crate) Client<R>);

impl<R: RetryStrategy> NetworkApi<R> {
    /// Get the current network name of the chain.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an `NetworkName` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_network_name(&self) -> Result<NetworkName, Error> {
        let resp: Response<NetworkName> = self.as_ref().send(Request::get_network_name()).await?;
        Ok((*resp).clone())
    }

    /// Get the Symbol network properties.
    ///
    /// # Info
    ///
    /// Properties from a catapult-server network configuration file (resources/config-network.properties).
    /// To enable this feature, the REST setting "network.propertiesFilePath" must define where the file is located.
    /// This is adjustable via the configuration file (rest/resources/rest.json) per REST instance.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an `NetworkName` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_network_properties(&self) -> Result<NetworkConfiguration, Error> {
        let resp: Response<NetworkConfiguration> = self
            .as_ref()
            .send(Request::get_network_properties())
            .await?;
        Ok((*resp).clone())
    }
}

impl<R: RetryStrategy> AsRef<Client<R>> for NetworkApi<R> {
    fn as_ref(&self) -> &Client<R> {
        &self.0
    }
}
