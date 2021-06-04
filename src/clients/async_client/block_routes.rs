use crate::blockchain::{BlockInfo, MerkleProofInfo};
use crate::clients::model_dto::{BlockInfoDto, MerkleProofInfoDto, BlockPageDto};
use crate::clients::search_criteria::block_search_criteria::BlockSearchCriteria;
use crate::{request::Request, Client, Error, Order, Response, RetryStrategy, H192, H256};

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
    pub async fn get_block_by_height(&self, height: u64) -> Result<BlockInfo<H192>, Error> {
        let resp: Response<BlockInfoDto> = self
            .as_ref()
            .send(Request::get_block_by_height(height))
            .await?;
        resp.to_compat()
            .map_err(|e| Error::unexpected_uncategorized(e.to_string()))
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
        resp.to_compact()
            .map_err(|e| Error::unexpected_uncategorized(e.to_string()))
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
        resp.to_compact()
            .map_err(|e| Error::unexpected_uncategorized(e.to_string()))
    }

    /// Gets an vec of blocks.
    ///
    /// # Inputs
    ///
    /// * `criteria`: Defines the params used to search blocks.
    /// * `pageSize`: Select the number of entries to return, (Default = 10).
    /// * `pageNumber`: Filter by page number, (Default = 1).
    /// * `offset`: Entry id at which to start pagination.
    ///     If the ordering parameter is set to -id, the elements returned precede the identifier.
    ///     Otherwise, newer elements with respect to the id are returned, (Default = "desc")
    /// * `order`: Sort responses in ascending or descending order based on the collection property set on the param orderBy.
    ///     If the request does not specify orderBy, REST returns the collection ordered by id, (Default: "desc").
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an `Vec<BlockInfo>` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub async fn search_blocks(
        &self,
        criteria: Option<BlockSearchCriteria>,
        page_size: Option<i32>,
        page_number: Option<i32>,
        offset: Option<&str>,
        order: Option<Order>,
    ) -> Result<Vec<BlockInfo<H192>>, Error> {
        let resp: Response<BlockPageDto> = self
            .as_ref()
            .send(Request::search_blocks(
                criteria,
                page_size,
                page_number,
                offset,
                order,
            ))
            .await?;
        resp.to_compact().map_err(|e| Error::unexpected_uncategorized(e.to_string()))
    }
}

impl<R: RetryStrategy> AsRef<Client<R>> for BlockApi<R> {
    fn as_ref(&self) -> &Client<R> {
        &self.0
    }
}
