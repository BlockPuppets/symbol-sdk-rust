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
use crate::buffer::*;
use crate::message::{Message, MessageType};
use crate::mosaic::Mosaic;
use crate::network::NetworkType;
use crate::transaction::common_transaction::CommonTransaction;
use crate::transaction::{Transaction, TransactionType, TransactionVersion};
use crate::{hex_decode, utf8_to_hex, Deadline};

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
    fn to_serializer(&self) -> Vec<u8> {
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

    use crate::account::{Address, PublicAccount};
    use crate::message::{
        PersistentHarvestingDelegationMessage, PlainMessage, PERSISTENT_DELEGATION_UNLOCK,
    };

    use super::*;
    use crate::account::tests::TESTING_ACCOUNT;
    use crate::model::namespace::NamespaceId;
    use crate::H256;
    use std::str::FromStr;

    const EPOCH_ADJUSTMENT: u64 = 1573430400;
    const DELEGATED_PRIVATE_KEY: &str =
        "8A78C9E9B0E59D0F74C0D47AB29FBD523C706293A3FA9CD9FE0EEB2C10EA924A";
    const VRF_PRIVATE_KEY: &str =
        "800F35F1CC66C2B62CE9DD9F31003B9B3E5C7A2F381FB8952A294277A1015D83";
    const RECIPIENT_PUBLIC_KEY: &str =
        "9DBF67474D6E1F8B131B4EB1F5BA0595AFFAE1123607BC1048F342193D7E669F";

    lazy_static! {
        static ref GENERATION_HASH: H256 =
            H256::from_str("57F7DA205008026C776CB6AED843393F04CD458E0AA2D9F1D5F31A402072B2D6")
                .unwrap();
        static ref MESSAGE_MARKER: &'static str = PERSISTENT_DELEGATION_UNLOCK;
    }

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

    #[test]
    fn should_create_complete_an_transfer_transaction_and_sign_it_without_mosaics() {
        let transfer_transaction = TransferTransaction::create(
            Deadline::create(EPOCH_ADJUSTMENT, Duration::hours(2)).unwrap(),
            Address::from_raw("VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ").unwrap(),
            vec![],
            PlainMessage::create("test-message"),
            NetworkType::PrivateTest,
            None,
        )
        .unwrap();

        assert_eq!(transfer_transaction.message.payload(), "test-message");
        assert_eq!(transfer_transaction.mosaics.len(), 0);
        assert!(transfer_transaction
            .recipient
            .try_downcast_ref::<Address>()
            .is_some());
        assert_eq!(
            transfer_transaction
                .recipient
                .downcast_ref::<Address>()
                .recipient_to_string(),
            "VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ"
        );

        let signed_transaction = transfer_transaction
            .sign_with(*TESTING_ACCOUNT, *GENERATION_HASH)
            .unwrap();

        assert_eq!(&signed_transaction.payload[256..signed_transaction.payload.len()], "A826D27E1D0A26CA4E316F901E23E55C8711DB20DF45C5360D0000000000000000746573742D6D657373616765");
    }

    #[test]
    fn should_create_complete_an_transfer_transaction_with_empty_message() {
        let transfer_transaction = TransferTransaction::create(
            Deadline::create(EPOCH_ADJUSTMENT, Duration::hours(2)).unwrap(),
            Address::from_raw("VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ").unwrap(),
            vec![],
            Message::empty_message(),
            NetworkType::PrivateTest,
            None,
        )
        .unwrap();

        assert_eq!(transfer_transaction.message.payload(), "");
        assert_eq!(transfer_transaction.mosaics.len(), 0);
        assert!(transfer_transaction
            .recipient
            .try_downcast_ref::<Address>()
            .is_some());
        assert_eq!(
            transfer_transaction
                .recipient
                .downcast_ref::<Address>()
                .recipient_to_string(),
            "VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ"
        );

        let signed_transaction = transfer_transaction
            .sign_with(*TESTING_ACCOUNT, *GENERATION_HASH)
            .unwrap();

        assert_eq!(
            &signed_transaction.payload[256..signed_transaction.payload.len()],
            "A826D27E1D0A26CA4E316F901E23E55C8711DB20DF45C5360000000000000000"
        );
    }

    #[test]
    fn should_create_complete_an_transfer_transaction_and_sign_it_with_mosaics() {
        let namespace_id = NamespaceId::create_from_name("cat.currency").unwrap();

        let mosaic = Mosaic::create_relative(namespace_id, 100, 6).unwrap();

        let transfer_transaction = TransferTransaction::create(
            Deadline::create(EPOCH_ADJUSTMENT, Duration::hours(2)).unwrap(),
            Address::from_raw("VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ").unwrap(),
            vec![mosaic],
            PlainMessage::create("test-message"),
            NetworkType::PrivateTest,
            None,
        )
        .unwrap();

        assert_eq!(transfer_transaction.message.payload(), "test-message");
        assert_eq!(transfer_transaction.mosaics.len(), 1);
        assert!(transfer_transaction
            .recipient
            .try_downcast_ref::<Address>()
            .is_some());
        assert_eq!(
            transfer_transaction
                .recipient
                .downcast_ref::<Address>()
                .recipient_to_string(),
            "VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ"
        );

        let signed_transaction = transfer_transaction
            .sign_with(*TESTING_ACCOUNT, *GENERATION_HASH)
            .unwrap();

        assert_eq!(&signed_transaction.payload[256..signed_transaction.payload.len()], "A826D27E1D0A26CA4E316F901E23E55C8711DB20DF45C5360D0001000000000044B262C46CEABB8500E1F5050000000000746573742D6D657373616765");
    }

    #[test]
    fn should_create_complete_an_transfer_transaction_with_namespace_id_recipient_address() {
        let address_alias = NamespaceId::create_from_name("nem.owner").unwrap();

        let namespace_id = NamespaceId::create_from_name("cat.currency").unwrap();

        let mosaic = Mosaic::create_relative(namespace_id, 100, 6).unwrap();

        let transfer_transaction = TransferTransaction::create(
            Deadline::create(EPOCH_ADJUSTMENT, Duration::hours(2)).unwrap(),
            address_alias.clone(),
            vec![mosaic],
            PlainMessage::create("test-message"),
            NetworkType::PrivateTest,
            None,
        )
        .unwrap();

        assert_eq!(transfer_transaction.message.payload(), "test-message");
        assert_eq!(transfer_transaction.mosaics.len(), 1);
        assert!(transfer_transaction
            .recipient
            .try_downcast_ref::<NamespaceId>()
            .is_some());
        assert_eq!(
            transfer_transaction.recipient.downcast_ref::<NamespaceId>(),
            &address_alias
        );
        assert_eq!(
            transfer_transaction
                .recipient
                .downcast_ref::<NamespaceId>()
                .to_hex(),
            address_alias.to_hex()
        );

        let signed_transaction = transfer_transaction
            .sign_with(*TESTING_ACCOUNT, *GENERATION_HASH)
            .unwrap();

        assert_eq!(&signed_transaction.payload[256..signed_transaction.payload.len()], "A951776168D24257D80000000000000000000000000000000D0001000000000044B262C46CEABB8500E1F5050000000000746573742D6D657373616765");
    }

    #[test]
    fn should_format_transfer_transaction_payload_with_24_bytes_binary_address() {
        let namespace_id = NamespaceId::create_from_name("cat.currency").unwrap();

        let mosaic = Mosaic::create_relative(namespace_id, 100, 6).unwrap();

        let transfer_transaction = TransferTransaction::create(
            Deadline::create(EPOCH_ADJUSTMENT, Duration::hours(2)).unwrap(),
            Address::from_raw("VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ").unwrap(),
            vec![mosaic],
            PlainMessage::create("test-message"),
            NetworkType::PrivateTest,
            None,
        )
        .unwrap();

        // test recipientToString with Address recipient
        assert_eq!(
            transfer_transaction.recipient_to_string(),
            "VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ"
        );

        let signed_transaction = transfer_transaction
            .sign_with(*TESTING_ACCOUNT, *GENERATION_HASH)
            .unwrap();

        assert_eq!(
            &signed_transaction.payload[256..306],
            "A826D27E1D0A26CA4E316F901E23E55C8711DB20DF45C5360D"
        );
    }

    #[test]
    fn should_format_transfer_transaction_payload_with_8_bytes_binary_namespace_id() {
        let namespace_id = NamespaceId::create_from_name("cat.currency").unwrap();

        let mosaic = Mosaic::create_relative(namespace_id, 100, 6).unwrap();

        let transfer_transaction = TransferTransaction::create(
            Deadline::create(EPOCH_ADJUSTMENT, Duration::hours(2)).unwrap(),
            NamespaceId::create_from_name("nem.owner").unwrap(),
            vec![mosaic],
            PlainMessage::create("test-message"),
            NetworkType::PrivateTest,
            None,
        )
        .unwrap();

        // test recipientToString with NamespaceId recipient
        assert_eq!(
            transfer_transaction.recipient_to_string(),
            "D85742D268617751"
        );

        let signed_transaction = transfer_transaction
            .sign_with(*TESTING_ACCOUNT, *GENERATION_HASH)
            .unwrap();

        assert_eq!(
            &signed_transaction.payload[256..306],
            "A951776168D24257D80000000000000000000000000000000D"
        );
    }

    #[test]
    fn should_create_transfer_transaction_for_persistent_harvesting_delegation_request_transaction()
    {
        let network_type = NetworkType::PrivateTest;

        let node_public_account =
            PublicAccount::from_public_key(RECIPIENT_PUBLIC_KEY, network_type).unwrap();
        let transfer_transaction = TransferTransaction::create(
            Deadline::create(EPOCH_ADJUSTMENT, Duration::hours(2)).unwrap(),
            Address::from_raw("VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ").unwrap(),
            vec![],
            PersistentHarvestingDelegationMessage::create(
                DELEGATED_PRIVATE_KEY,
                VRF_PRIVATE_KEY,
                node_public_account,
            ),
            network_type,
            None,
        )
        .unwrap();

        assert_eq!(
            transfer_transaction.message.message_type(),
            MessageType::PersistentHarvestingDelegationMessageType
        );
    }

    #[test]
    fn should_create_complete_an_persistent_delegation_request_transaction_and_sign_it() {
        let network_type = NetworkType::PrivateTest;

        let node_public_account =
            PublicAccount::from_public_key(RECIPIENT_PUBLIC_KEY, network_type).unwrap();
        let transfer_transaction = TransferTransaction::create(
            Deadline::create(EPOCH_ADJUSTMENT, Duration::hours(2)).unwrap(),
            Address::from_raw("VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ").unwrap(),
            vec![],
            PersistentHarvestingDelegationMessage::create(
                DELEGATED_PRIVATE_KEY,
                VRF_PRIVATE_KEY,
                node_public_account,
            ),
            network_type,
            None,
        )
        .unwrap();

        println!("{}", transfer_transaction.message.payload());
        assert_eq!(
            transfer_transaction.message.payload().len(),
            248 + MESSAGE_MARKER.len()
        );
        assert!(transfer_transaction
            .message
            .payload()
            .contains(MESSAGE_MARKER.clone()));
        assert_eq!(transfer_transaction.mosaics.len(), 0);
        assert!(transfer_transaction
            .recipient
            .try_downcast_ref::<Address>()
            .is_some());
        assert_eq!(
            transfer_transaction
                .recipient
                .downcast_ref::<Address>()
                .recipient_to_string(),
            "VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ"
        );

        let signed_transaction = transfer_transaction
            .sign_with(*TESTING_ACCOUNT, *GENERATION_HASH)
            .unwrap();

        assert!(
            &signed_transaction.payload[256..signed_transaction.payload.len()]
                .contains(&transfer_transaction.message.payload())
        );
    }

    pub mod tests_size {
        use super::*;

        #[test]
        fn should_return_180_for_transfer_transaction_with_1_mosaic_and_message_nem() {
            let namespace_id = NamespaceId::create_from_name("cat.currency").unwrap();

            let mosaic = Mosaic::create_relative(namespace_id, 100, 6).unwrap();

            let transfer_transaction = TransferTransaction::create(
                Deadline::create(EPOCH_ADJUSTMENT, Duration::hours(2)).unwrap(),
                Address::from_raw("VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ").unwrap(),
                vec![mosaic],
                PlainMessage::create("NEM"),
                NetworkType::PrivateTest,
                None,
            )
            .unwrap();

            assert_eq!(
                transfer_transaction.to_serializer().len(),
                transfer_transaction.size()
            );
            assert_eq!(transfer_transaction.size(), 180);
        }
    }
}
