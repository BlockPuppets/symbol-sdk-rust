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
use crate::buffer::*;
use crate::network::NetworkType;
use crate::transaction::{TransactionInfo, TransactionType, TransactionVersion};
use crate::{hex_decode, Deadline, H256};
use std::convert::TryFrom;

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

    pub fn get_transaction_hash(&self) -> H256 {
        match self.transaction_info.to_owned() {
            Some(h) => match h.hash {
                Some(hs) => hs,
                _ => H256::default(),
            },
            _ => H256::default(),
        }
    }

    /// Converts the optional signer to a KeyDto that can be serialized.
    pub(crate) fn __get_signer_as_builder(&self) -> key_dto::KeyDto {
        if let Some(signer) = self.signer {
            signer.to_builder()
        } else {
            key_dto::KeyDto([0u8; 32])
        }
    }

    /// Converts the optional signer to a KeyDto that can be serialized.
    pub(crate) fn __get_signature_as_builder(&self) -> signature_dto::SignatureDto {
        if let Some(ref signature) = self.signature {
            signature_dto::SignatureDto(<[u8; 64]>::try_from(hex_decode(&signature)).unwrap())
        } else {
            signature_dto::SignatureDto([0u8; 64])
        }
    }

    pub(crate) fn __version_to_dto(&self) -> u8 {
        (((self.network_type.value() as u32) << 8) + *self.version as u32) as u8
    }

    pub fn common_builder(&self) -> transaction_builder::TransactionBuilder {
        transaction_builder::TransactionBuilder {
            signature: self.__get_signature_as_builder(),
            signer_public_key: self.__get_signer_as_builder(),
            version: self.__version_to_dto(),
            network: self.network_type.to_builder(),
            _type: self.transaction_type.to_builder(),
            fee: amount_dto::AmountDto(self.max_fee),
            deadline: timestamp_dto::TimestampDto(*self.deadline),
        }
    }

    pub fn common_embedded_builder(
        &self,
    ) -> embedded_transaction_builder::EmbeddedTransactionBuilder {
        embedded_transaction_builder::EmbeddedTransactionBuilder {
            signer_public_key: self.__get_signer_as_builder(),
            version: self.__version_to_dto(),
            network: self.network_type.to_builder(),
            _type: self.transaction_type.to_builder(),
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
