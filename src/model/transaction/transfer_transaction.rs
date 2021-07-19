/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::fmt;

use anyhow::Result;

use crate::account::UnresolvedAddress;
use crate::message::Message;
use crate::mosaic::Mosaic;
use crate::network::NetworkType;
use crate::transaction::common_transaction::CommonTransaction;
use crate::transaction::{Transaction, TransactionType, TransactionVersion};
use crate::Deadline;

/// Create a transfer transaction struct.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferTransaction {
    pub common: CommonTransaction,
    /// The `Address` of the recipient address.
    pub recipient: Box<dyn UnresolvedAddress>,
    /// The vec of Mosaic.
    pub mosaics: Vec<Mosaic>,
    /// The transaction message of 2048 characters.
    pub message: Box<dyn Message>,
}

impl TransferTransaction {
    /// Create a transfer transaction object.
    pub fn create<M: 'static + Message, R: 'static + UnresolvedAddress>(
        deadline: Deadline,
        recipient: R,
        mosaics: Vec<Mosaic>,
        message: M,
        network_type: NetworkType,
        max_fee: Option<u64>,
    ) -> Result<Self> {
        let max_fee = max_fee.unwrap_or_default();

        let common = CommonTransaction::create_from_type(
            TransactionType::Transfer,
            network_type,
            TransactionVersion::TRANSFER,
            deadline,
            max_fee,
        );

        Ok(Self {
            common,
            recipient: Box::new(recipient),
            mosaics,
            message: Box::new(message),
        })
    }

    /// The String notation for the set recipient.
    pub fn recipient_to_string(&self) -> String {
        todo!()
    }

    /// Sorted mosaic vec.
    pub fn sort_mosaics(&mut self) -> Vec<Mosaic> {
        self.mosaics.sort_by(|a, b| {
            let long_a = a.id.to_uint64();
            let long_b = b.id.to_uint64();
            long_a.cmp(&long_b)
        });

        self.mosaics.clone()
    }
}

#[typetag::serde]
impl Transaction for TransferTransaction {}

impl fmt::Display for TransferTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

#[cfg(test)]
pub mod tests {
    fn test_should_create_transfer_transaction() {
        todo!()
    }
}
