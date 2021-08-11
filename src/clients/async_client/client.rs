/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::convert::TryInto;
use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::clients::{model_dto::BlockInfoDto, retry::RetryStrategy, Error, SymbolResponse};
use crate::network::NetworkType;
use crate::{BlockApi, ChainApi, GenerationHash, MosaicApi, NetworkApi, NodeApi};

use super::{request::Request, HttpClient, Response, SimpleHttpClient};

#[derive(Clone)]
pub struct Client<R> {
    pub(crate) http_client: Arc<dyn HttpClient>,
    pub(crate) retry: R,
    pub generation_hash: GenerationHash,
    pub network_type: NetworkType,
}

impl<R: RetryStrategy> Client<R> {
    pub async fn from_url<T: reqwest::IntoUrl>(server_url: T, retry: R) -> Result<Self, Error> {
        let http_client =
            Arc::new(SimpleHttpClient::new(server_url).map_err(Error::InvalidHTTPResponse)?);

        let ret = http_client
            .as_ref()
            .single_request(&Request::get_block_by_height(1))
            .await?;

        let info = BlockInfoDto::deserialize(ret.result.unwrap())
            .map_err(Error::DeserializeResponseJsonError)?;

        let info = info
            .to_compact()
            .map_err(|e| Error::unexpected_uncategorized(e.to_string()))?;

        Ok(Self {
            http_client,
            generation_hash: info.generation_hash,
            network_type: info.network_type,
            retry,
        })
    }

    pub(crate) async fn send<T: DeserializeOwned>(
        &self,
        request: Request,
    ) -> Result<Response<T>, Error> {
        self.send_with_retry(&request, &self.retry)
            .await?
            .try_into()
    }

    async fn send_with_retry<RS: RetryStrategy>(
        &self,
        request: &Request,
        retry: &RS,
    ) -> Result<SymbolResponse, Error> {
        let mut retries: u32 = 0;
        loop {
            let ret = self.http_client.single_request(request).await;
            match ret {
                Ok(r) => return Ok(r),
                Err(err) => {
                    if let Error::SymbolError(_) = err {
                        return Err(err);
                    }
                    retries = self.handle_retry_error(retries, err, retry).await?
                }
            }
        }
    }

    async fn handle_retry_error<RS: RetryStrategy>(
        &self,
        mut retries: u32,
        err: Error,
        retry: &RS,
    ) -> Result<u32, Error> {
        // if !retry.is_retriable(&err) {
        //     return Err(err);
        // }
        if retries < retry.max_retries(&err) {
            match retries.checked_add(1) {
                Some(i) if i < retry.max_retries(&err) => {
                    retries = i;
                }
                _ => return Err(err),
            };
            tokio::time::sleep(retry.delay(&err, retries)).await;
            Ok(retries)
        } else {
            Err(err)
        }
    }
}

// routes api
impl<R: RetryStrategy> Client<R> {
    /// Symbol client block routes api.
    pub fn block_routes(&self) -> BlockApi<R> {
        BlockApi(self.clone())
    }

    /// Symbol client chain routes api.
    pub fn chain_routes(&self) -> ChainApi<R> {
        ChainApi(self.clone())
    }

    /// Symbol client network routes api.
    pub fn network_routes(&self) -> NetworkApi<R> {
        NetworkApi(self.clone())
    }

    /// Symbol client node routes api.
    pub fn node_routes(&self) -> NodeApi<R> {
        NodeApi(self.clone())
    }

    /// Symbol client namespace routes api.
    pub fn namespace_routes(&self) {
        todo!()
    }

    /// Symbol client mosaic routes api.
    pub fn mosaic_routes(&self) -> MosaicApi<R> {
        MosaicApi(self.clone())
    }

    /// Symbol client account routes api.
    pub fn account_routes(&self) {
        todo!()
    }

    /// Symbol client finalization routes api.
    pub fn finalization_routes(&self) {
        todo!()
    }

    /// Symbol client hash_lock routes api.
    pub fn hash_lock_routes(&self) {
        todo!()
    }

    /// Symbol client metadata routes api.
    pub fn metadata_routes(&self) {
        todo!()
    }

    /// Symbol client multisig routes api.
    pub fn multisig_routes(&self) {
        todo!()
    }

    /// Symbol client receipt routes api.
    pub fn receipt_routes(&self) {
        todo!()
    }

    /// Symbol client restriction routes api.
    pub fn restriction_account_routes(&self) {
        todo!()
    }

    /// Symbol client restriction routes api.
    pub fn restriction_mosaic_routes(&self) {
        todo!()
    }

    /// Symbol client secret_lock routes api.
    pub fn secret_lock_routes(&self) {
        todo!()
    }

    /// Symbol client transaction routes api.
    pub fn transaction_routes(&self) {
        todo!()
    }

    /// Symbol client transaction_status routes api.
    pub fn transaction_status_routes(&self) {
        todo!()
    }
}
