/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::{BlockSearchCriteria, H256, RetryStrategy};
use crate::blockchain::{BlockInfo, MerkleProofInfo};
use crate::clients::{
    Error,
    model_dto::{BlockInfoDto, BlockPageDto, MerkleProofInfoDto},
};

use super::{Client, request::Request, Response};

pub struct BlockApi<R: RetryStrategy>(pub(crate) Client<R>);

impl<R: RetryStrategy> BlockApi<R> {
    /// Gets a `BlockInfo` from the chain that has the given height.
    /// # Inputs
    ///
    /// * `height`: The Block height.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an `BlockInfo` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_block_by_height(&self, height: u64) -> Result<BlockInfo, Error> {
        let resp: Response<BlockInfoDto> = self
            .as_ref()
            .send(Request::get_block_by_height(height))
            .await?;
        resp.to_compact().map_err(Into::into)
    }

    /// Get the merkle path for a given a receipt statement hash and block.
    ///
    /// # Note
    ///
    /// Returns the merkle path for a [receipt statement or resolution](https://nemtech.github.io/concepts/receipt.html)
    /// linked to a block. The path is the complementary data needed to calculate the merkle root.
    /// A client can compare if the calculated root equals the one recorded in the block header,
    /// verifying that the receipt was linked with the block.
    ///
    /// # Inputs
    ///
    /// * `height`: The height of the block.
    /// * `hash`: The hash of the receipt statement or resolution.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an `MerkleProofInfo` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_merkle_receipts(
        &self,
        height: u64,
        hash: H256,
    ) -> Result<MerkleProofInfo, Error> {
        let resp: Response<MerkleProofInfoDto> = self
            .as_ref()
            .send(Request::get_merkle_receipts(height, hash))
            .await?;
        resp.to_compact().map_err(Into::into)
    }

    /// Get the merkle path for a given a transaction and block.
    ///
    /// # Note
    ///
    /// Returns the merkle path for a [transaction](https://nemtech.github.io/concepts/transaction.html)
    /// included in a block. The path is the complementary data needed to calculate the merkle root.
    /// A client can compare if the calculated root equals the one recorded in the block header,
    /// verifying that the transaction was included in the block.
    ///
    /// # Inputs
    ///
    /// * `height:` The height of the block.
    /// * `hash`: The hash of the transaction.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an `MerkleProofInfo` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn get_merkle_transaction(
        &self,
        height: u64,
        hash: H256,
    ) -> Result<MerkleProofInfo, Error> {
        let resp: Response<MerkleProofInfoDto> = self
            .as_ref()
            .send(Request::get_merkle_transaction(height, hash))
            .await?;
        resp.to_compact().map_err(Into::into)
    }

    /// Gets an vec of blocks.
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
    pub async fn search_blocks(
        &self,
        criteria: Option<BlockSearchCriteria>,
    ) -> Result<Vec<BlockInfo>, Error> {
        let resp: Response<BlockPageDto> =
            self.as_ref().send(Request::search_blocks(criteria)).await?;
        resp.to_compact().map_err(Into::into)
    }
}

impl<R: RetryStrategy> AsRef<Client<R>> for BlockApi<R> {
    fn as_ref(&self) -> &Client<R> {
        &self.0
    }
}
