/// The blockchain storage info structure describes stored data.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct StorageInfo {
    /// The number of confirmed blocks.
    pub num_blocks: usize,

    /// The number of confirmed transactions.
    pub num_transactions: usize,

    /// The number accounts published in the blockchain.
    pub num_accounts: usize,
}
