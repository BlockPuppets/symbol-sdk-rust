/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::account::PublicAccount;
use crate::network::NetworkType;
use crate::transaction::{TransactionInfo, TransactionType, TransactionVersion};
use crate::{Deadline, H256};

pub type Height = u64;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CommonTransaction {
    /// The transaction type.
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,

    /// The network type.
    pub network_type: NetworkType,

    /// The transaction version number.
    pub version: TransactionVersion,

    /// The deadline to include the transaction.
    pub deadline: Deadline,

    /// A sender of a transaction must specify during the transaction definition a max_fee,
    /// meaning the maximum fee the account allows to spend for this transaction.
    pub max_fee: u64,

    /// The transaction signature (missing if part of an aggregate transaction).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    /// The account of the transaction creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer: Option<PublicAccount>,

    /// Transactions meta data object contains additional information about the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_info: Option<TransactionInfo>,
}

impl CommonTransaction {
    pub fn create_from_type(
        transaction_type: TransactionType,
        network_type: NetworkType,
        version: TransactionVersion,
        deadline: Deadline,
        max_fee: u64,
    ) -> Self {
        CommonTransaction {
            transaction_info: None,
            network_type,
            signature: Default::default(),
            signer: Default::default(),
            version,
            transaction_type,
            max_fee,
            deadline,
        }
    }

    pub fn get_hash(&self) -> H256 {
        match self.transaction_info.to_owned() {
            Some(h) => match h.hash {
                Some(hs) => hs,
                _ => H256::default(),
            },
            _ => H256::default(),
        }
    }
}

impl core::fmt::Display for CommonTransaction {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
