/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::{Client, Error, Response, RetryStrategy};
use crate::clients::request::Request;
use crate::model_dto::RentalFeesDto;
use crate::network::{NetworkConfiguration, NetworkName, NetworkType, RentalFees, TransactionFees};

pub struct NetworkApi<R: RetryStrategy>(pub(crate) Client<R>);

impl<R: RetryStrategy> NetworkApi<R> {
    /// Get current network type.
    ///
    pub async fn get_network_type(&self) -> NetworkType {
        self.as_ref().network_type
    }

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

    /// Get rental fees information.
    ///
    /// # Info
    /// This endpoint is only available if the REST instance has access to catapult-server resources/config-network.properties file.
    /// To activate this feature, add the setting "network.propertiesFilePath" in the configuration file (rest/resources/rest.json).
    ///
    /// # Returns
    ///
    /// Returns the estimated effective rental fees for namespaces and mosaics.
    /// A `Result` whose okay value is an `RentalFees` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_rental_fees(&self) -> Result<RentalFees, Error> {
        let resp: Response<RentalFeesDto> = self.as_ref().send(Request::get_rental_fees()).await?;
        resp.to_compact()
            .map_err(Into::into)
    }

    /// Get transaction fees information.
    ///
    /// # Returns
    ///
    /// Returns the average, median, highest and lower fee multiplier over the last "numBlocksTransactionFeeStats".
    /// The setting "numBlocksTransactionFeeStats" is adjustable via the configuration file (rest/resources/rest.json) per REST instance.
    /// A `Result` whose okay value is an `TransactionFees` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_transaction_fees(&self) -> Result<TransactionFees, Error> {
        let resp: Response<TransactionFees> =
            self.as_ref().send(Request::get_transaction_fees()).await?;
        Ok((*resp).clone())
    }
}

impl<R: RetryStrategy> AsRef<Client<R>> for NetworkApi<R> {
    fn as_ref(&self) -> &Client<R> {
        &self.0
    }
}
