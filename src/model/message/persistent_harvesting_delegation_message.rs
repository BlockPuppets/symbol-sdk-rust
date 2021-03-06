/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use anyhow::{ensure, Result};

use crate::account::{Account, PublicAccount};
use crate::message::{EncryptedMessage, Message, MessageType};
use crate::{hex_decode, is_hex};

/// 8-byte marker: FE2A8061577301E2 for `PersistentHarvestingDelegationMessage`
pub(crate) const PERSISTENT_DELEGATION_UNLOCK: &'static str = "FE2A8061577301E2";

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PersistentHarvestingDelegationMessage {
    /// Message type
    pub r#type: MessageType,
    /// Message payload
    pub payload: String,
}

impl PersistentHarvestingDelegationMessage {
    pub(crate) const HEX_PAYLOAD_SIZE: usize = 264;

    /// Create new `PersistentHarvestingDelegationMessage`.
    ///
    pub fn new<C>(
        remote_linked_private_key: &str,
        vrf_private_key: &str,
        node_public_account: PublicAccount,
    ) -> Self {
        let ephemeral_keypair = Account::random(node_public_account.network_type());

        let data = remote_linked_private_key.to_string() + vrf_private_key;
        let mut encrypted = String::new();
        encrypted.push_str(PERSISTENT_DELEGATION_UNLOCK);
        encrypted.push_str(&ephemeral_keypair.public_key_to_hex());
        encrypted.push_str(
            &ephemeral_keypair
                .encrypt_message(data, node_public_account)
                .unwrap()
                .payload,
        );

        Self {
            r#type: MessageType::PersistentHarvestingDelegationMessageType,
            payload: encrypted,
        }
    }

    /// Create `PersistentHarvestingDelegationMessage` from DTO payload with marker.
    ///
    pub fn from_payload(payload: &str) -> Result<Self> {
        let payload = payload.to_uppercase();

        ensure!(
            is_hex(&payload),
            "Payload format is not valid hexadecimal string"
        );
        ensure!(
            payload.len() == Self::HEX_PAYLOAD_SIZE,
            format!(
                "Invalid persistent harvesting delegate payload size! Expected {} but got {}",
                Self::HEX_PAYLOAD_SIZE,
                payload.len()
            )
        );
        ensure!(
            payload.find(PERSISTENT_DELEGATION_UNLOCK).is_some(),
            format!(
                "Invalid persistent harvesting delegate payload! It does not start with {}",
                PERSISTENT_DELEGATION_UNLOCK
            )
        );

        Ok(Self {
            r#type: MessageType::PersistentHarvestingDelegationMessageType,
            payload,
        })
    }

    /// Encrypted message to be decrypted.
    ///
    pub fn decrypt(&self, private_account: Account) -> Result<String> {
        let marker_length = PERSISTENT_DELEGATION_UNLOCK.len();
        let ephemeral_public_key = &self.payload[marker_length..marker_length + 64];
        let payload = hex_decode(&self.payload[marker_length + 64..]);
        let ephemeral_public_account =
            PublicAccount::from_public_key(ephemeral_public_key, private_account.network_type())?;

        let decrypt_message = private_account.decrypt_message(
            &EncryptedMessage::from_bytes(&payload)?,
            ephemeral_public_account,
        )?;

        Ok(decrypt_message.payload)
    }
}

#[typetag::serde]
impl Message for PersistentHarvestingDelegationMessage {
    fn message_type(&self) -> MessageType {
        self.r#type
    }
    fn payload(&self) -> String {
        self.payload.to_owned()
    }
    fn box_clone(&self) -> Box<dyn Message + 'static> {
        Box::new((*self).clone())
    }
}
