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

use crate::{Deadline, hex_decode, utf8_to_hex};
use crate::account::UnresolvedAddress;
use crate::buffer::*;
use crate::message::{Message, MessageType};
use crate::mosaic::Mosaic;
use crate::network::NetworkType;
use crate::transaction::{Transaction, TransactionType, TransactionVersion};
use crate::transaction::common_transaction::CommonTransaction;

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
        self.recipient.to_string()
    }

    /// Sorted mosaic vec.
    pub fn sort_mosaics(&self) -> Vec<Mosaic> {
        let mut mosaics_clone = self.mosaics.clone();
        mosaics_clone.sort_by(|a, b| {
            let long_a = a.id.to_uint64();
            let long_b = b.id.to_uint64();
            long_a.cmp(&long_b)
        });
        mosaics_clone
    }

    pub fn to_transaction_builder(
        &self,
    ) -> transfer_transaction_builder::TransferTransactionBuilder {
        transfer_transaction_builder::TransferTransactionBuilder {
            super_object: self.common.common_builder(),
            body: self.__to_transaction_body_builder(),
        }
    }

    // internal
    fn __to_transaction_body_builder(
        &self,
    ) -> transfer_transaction_body_builder::TransferTransactionBodyBuilder {
        transfer_transaction_body_builder::TransferTransactionBodyBuilder {
            recipient_address: buffer::unresolved_address_dto::UnresolvedAddressDto::from_binary(
                &self
                    .recipient
                    .unresolved_address_to_bytes(self.common.network_type),
            ),
            mosaics: self.sort_mosaics().iter().map(|m| m.to_builder()).collect(),
            message: self.__get_message_buffer(),
        }
    }

    // internal
    fn __get_message_buffer(&self) -> Vec<u8> {
        if self.message.to_vec().is_empty() || self.message.payload_to_vec().is_empty() {
            return vec![];
        }
        let message_hex = if self.message.message_type()
            == MessageType::PersistentHarvestingDelegationMessageType
        {
            self.message.payload()
        } else {
            utf8_to_hex(&*self.message.payload())
        };

        let payload_buffer = hex_decode(&*message_hex);
        let type_buffer = self.message.message_type().to_bytes();

        if self.message.message_type() == MessageType::PersistentHarvestingDelegationMessageType
            || self.message.payload_to_vec().is_empty()
        {
            payload_buffer
        } else {
            [type_buffer.as_ref(), payload_buffer.as_ref()].concat()
        }
    }
}

#[typetag::serde]
impl Transaction for TransferTransaction {
    fn serializer(&self) -> Vec<u8> {
        self.to_transaction_builder().serializer()
    }

    fn get_common_transaction(&self) -> &CommonTransaction {
        &self.common
    }

    fn to_embedded_transaction_builder(
        &self,
    ) -> Box<dyn embedded_transaction_helper::EmbeddedTransactionHelper> {
        Box::new(
            embedded_transfer_transaction_builder::EmbeddedTransferTransactionBuilder {
                super_object: self.common.common_embedded_builder(),
                body: self.__to_transaction_body_builder(),
            },
        )
    }
}

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
    use chrono::Duration;

    use crate::account::Address;
    use crate::message::PlainMessage;

    use super::*;

    const EPOCH_ADJUSTMENT: u64 = 1573430400;

    #[test]
    fn should_default_max_fee_field_be_set_to_0() {
        let transfer_transaction = TransferTransaction::create(
            Deadline::create(EPOCH_ADJUSTMENT, Duration::hours(2)).unwrap(),
            Address::from_raw("VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ").unwrap(),
            vec![],
            PlainMessage::create("test-message"),
            NetworkType::PrivateTest,
            None,
        )
            .unwrap();

        assert_eq!(transfer_transaction.common.max_fee, 0);
    }

    #[test]
    fn should_filled_max_fee_override_transaction_max_fee() {
        let transfer_transaction = TransferTransaction::create(
            Deadline::create(EPOCH_ADJUSTMENT, Duration::hours(2)).unwrap(),
            Address::from_raw("VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ").unwrap(),
            vec![],
            PlainMessage::create("test-message"),
            NetworkType::PrivateTest,
            Some(1),
        )
            .unwrap();

        assert_eq!(transfer_transaction.common.max_fee, 1);
    }
}
