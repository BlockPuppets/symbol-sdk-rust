/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::account::{Address, PublicAccount};
use crate::model::network::NetworkType;
use crate::model::transaction::TransactionType;
use std::fmt;
use crate::H256;

/// SignedTransaction object is used to transfer the transaction data and the signature to the server in order to initiate and broadcast a transaction.
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedTransaction {
    /// Transaction serialized data
    pub payload: String,
    /// Transaction hash
    pub hash: H256,
    /// Transaction signerPublicKey
    pub signer_public_key: PublicAccount,
    /// Transaction type
    #[serde(rename = "type")]
    pub _type: TransactionType,
    /// Signer network type
    pub network_type: NetworkType,
}

impl SignedTransaction {

    /// Get signer `Address`.
    pub fn get_signer_address(&self) -> Address {
        self.signer_public_key.address
    }
}

impl fmt::Display for SignedTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}