/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::{Client, Error, MosaicSearchCriteria, Response, RetryStrategy};
use crate::blockchain::MerkleStateInfo;
use crate::clients::request::Request;
use crate::model_dto::{MerkleStateInfoDto, MosaicInfoDto, MosaicPageDto};
use crate::mosaic::{MosaicId, MosaicInfo};

pub struct MosaicApi<R: RetryStrategy>(pub(crate) Client<R>);

impl<R: RetryStrategy> MosaicApi<R> {
    /// Gets the MosaicInfo for a given mosaicId.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an `MosaicInfo` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_mosaic_merkle(&self, mosaic_id: MosaicId) -> Result<MerkleStateInfo, Error> {
        let resp: Response<MerkleStateInfoDto> = self
            .as_ref()
            .send(Request::get_mosaic_merkle(mosaic_id))
            .await?;
        resp.to_compact()
            .map_err(|e| Error::unexpected_uncategorized(e.to_string()))
    }

    /// Gets the MosaicInfo for a given mosaicId.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an `MosaicInfo` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_mosaic(&self, mosaic_id: MosaicId) -> Result<MosaicInfo, Error> {
        let resp: Response<MosaicInfoDto> =
            self.as_ref().send(Request::get_mosaic(mosaic_id)).await?;
        resp.to_compact()
            .map_err(|e| Error::unexpected_uncategorized(e.to_string()))
    }

    /// Gets the MosaicInfo for a given mosaicId.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an `MosaicInfo` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_mosaics(&self, mosaic_ids: Vec<MosaicId>) -> Result<Vec<MosaicInfo>, Error> {
        let resp: Response<Vec<MosaicInfoDto>> = self
            .as_ref()
            .send(Request::get_mosaics(mosaic_ids.into()))
            .await?;

        let mut mosaics = vec![];
        for mosaic in resp.result {
            mosaics.push(
                mosaic
                    .to_compact()
                    .map_err(|e| Error::unexpected_uncategorized(e.to_string()))?,
            )
        }
        Ok(mosaics)
    }

    /// Gets an vec of `MosaicInfo`.
    ///
    /// # Inputs
    ///
    /// * `criteria`: Defines the params used to search blocks.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an `Vec<BlockInfo>` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn search_mosaics(
        &self,
        criteria: Option<MosaicSearchCriteria>,
    ) -> Result<Vec<MosaicInfo>, Error> {
        let resp: Response<MosaicPageDto> = self
            .as_ref()
            .send(Request::search_mosaics(criteria))
            .await?;
        resp.to_compact()
            .map_err(|e| Error::unexpected_uncategorized(e.to_string()))
    }
}

impl<R: RetryStrategy> AsRef<Client<R>> for MosaicApi<R> {
    fn as_ref(&self) -> &Client<R> {
        &self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct MosaicIds {
    /// The vec of mosaic identifiers.
    #[serde(rename = "mosaicIds")]
    pub mosaic_ids: Vec<String>,
}

impl From<Vec<MosaicId>> for MosaicIds {
    fn from(e: Vec<MosaicId>) -> Self {
        let ids = e.into_iter().map(|m| m.to_hex()).collect();
        Self { mosaic_ids: ids }
    }
}
