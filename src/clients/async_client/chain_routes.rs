/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::{Error, RetryStrategy};
use crate::blockchain::ChainInfo;
use crate::clients::{model_dto::ChainInfoDto, request::Request};

use super::{Client, Response};

pub struct ChainApi<R: RetryStrategy>(pub(crate) Client<R>);

impl<R: RetryStrategy> ChainApi<R> {
    /// Get the current information of the blockchain.
    ///
    /// # Info
    /// The higher the score, the better the chain. During synchronization,
    /// nodes try to get the best blockchain in the network.
    ///
    /// The score for a block is derived from its difficulty and the time (in seconds) that has elapsed since the last block:
    /// * block score = difficulty − time elapsed since last block
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an `ChainInfo` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_chain_info(&self) -> Result<ChainInfo, Error> {
        let resp: Response<ChainInfoDto> = self.as_ref().send(Request::get_chain_info()).await?;
        resp.to_compact().map_err(Into::into)
    }
}

impl<R: RetryStrategy> AsRef<Client<R>> for ChainApi<R> {
    fn as_ref(&self) -> &Client<R> {
        &self.0
    }
}
