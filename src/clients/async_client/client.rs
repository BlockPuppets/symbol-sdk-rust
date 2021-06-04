use std::convert::TryInto;
use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::clients::{model_dto::BlockInfoDto, retry::RetryStrategy, Error, JsonResponse};
use crate::network::NetworkType;
use crate::{BlockApi, GenerationHash};

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
            .to_compat()
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
    ) -> Result<JsonResponse, Error> {
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
    pub fn block_routes(&self) -> BlockApi<R> {
        BlockApi(self.clone())
    }

    pub fn chain_routes(&self) {
        todo!()
    }

    pub fn network_routes(&self) {
        todo!()
    }

    pub fn node_routes(&self) {
        todo!()
    }

    pub fn namespace_routes(&self) {
        todo!()
    }

    pub fn mosaic_routes(&self) {
        todo!()
    }

    pub fn account_routes(&self) {
        todo!()
    }

    pub fn finalization_routes(&self) {
        todo!()
    }

    pub fn hash_lock_routes(&self) {
        todo!()
    }

    pub fn metadata_routes(&self) {
        todo!()
    }

    pub fn multisig_routes(&self) {
        todo!()
    }

    pub fn receipt_routes(&self) {
        todo!()
    }

    pub fn restriction_account_routes(&self) {
        todo!()
    }

    pub fn restriction_mosaic_routes(&self) {
        todo!()
    }

    pub fn secret_lock_routes(&self) {
        todo!()
    }

    pub fn transaction_routes(&self) {
        todo!()
    }

    pub fn transaction_status_routes(&self) {
        todo!()
    }
}
