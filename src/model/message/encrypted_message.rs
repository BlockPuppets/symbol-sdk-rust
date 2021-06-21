/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::str::FromStr;

use anyhow::{ensure, Result};
use crypto::prelude;

use crate::is_hex;
use crate::message::{Message, MessageType, PlainMessage};

/// The `EncryptedMessage` struct defines a encrypted message string.
/// When sending it to the network we transform the payload to hex-string.
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct EncryptedMessage {
    /// Message type
    pub r#type: MessageType,
    /// Message payload.
    pub payload: String,
}

impl EncryptedMessage {
    /// Create a new `EncryptedMessage`.
    ///
    pub fn create(
        message: &[u8],
        signer_private_key: &str,
        recipient_public_key: &str,
    ) -> Result<Self> {
        ensure!(!message.is_empty(), "message must not be empty.");

        let signer_private_key = prelude::PrivateKey::from_str(signer_private_key)?;
        let recipient_public_key = prelude::PublicKey::from_str(recipient_public_key)?;

        let encrypt_message = signer_private_key.encrypt_message::<prelude::CryptoSym>(
            recipient_public_key.as_fixed_bytes(),
            message,
        )?;

        Ok(Self {
            r#type: MessageType::SecureMessageType,
            payload: hex::encode(encrypt_message).to_uppercase(),
        })
    }

    /// It creates a `EncryptedMessage` from the payload hex wihtout the 01 prefix.
    /// The 01 prefix will be attached to the final payload.
    ///
    pub fn from_payload(payload_hex: &str) -> Result<Self> {
        ensure!(is_hex(payload_hex), "payload_hex it's not hex.");
        Ok(Self {
            r#type: MessageType::SecureMessageType,
            payload: payload_hex.to_owned(),
        })
    }

    /// It creates a `EncryptedMessage` from the given bytes.
    ///
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        ensure!(!bytes.is_empty(), "bytes must not be empty.");
        let payload = hex::encode(bytes);
        Self::from_payload(&payload)
    }

    /// Encrypted message to be decrypted
    pub fn decrypt(
        &self,
        recipient_private_key: &str,
        signer_public_key: &str,
    ) -> Result<PlainMessage> {
        let recipient_private_key = crypto::prelude::PrivateKey::from_str(recipient_private_key)?;
        let signer_public_key = crypto::prelude::PublicKey::from_str(signer_public_key)?;

        let decrypted_message = recipient_private_key.decrypt_message::<prelude::CryptoSym>(
            signer_public_key.as_fixed_bytes(),
            &self.payload_to_vec(),
        )?;

        PlainMessage::from_bytes(&decrypted_message)
    }
}

#[typetag::serde]
impl Message for EncryptedMessage {
    fn message_type(&self) -> MessageType {
        self.r#type
    }
    fn payload(&self) -> String {
        self.payload.to_owned()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::account::Account;
    use crate::message::EncryptedMessage;
    use crate::network::NetworkType;

    const SENDER_PRIVATE_KEY: &str =
        "2602F4236B199B3DF762B2AAB46FC3B77D8DDB214F0B62538D3827576C46C108";
    const RECIPIENT_PRIVATE_KEY: &str =
        "B72F2950498111BADF276D6D9D5E345F04E0D5C9B8342DA983C3395B4CF18F08";

    lazy_static! {
        static ref SENDER: Account =
            Account::from_hex_private_key(SENDER_PRIVATE_KEY, NetworkType::PRIVATE_TEST).unwrap();
        static ref RECIPIENT: Account =
            Account::from_hex_private_key(RECIPIENT_PRIVATE_KEY, NetworkType::PRIVATE_TEST)
                .unwrap();
    }

    #[test]
    fn test_should_create_from_a_dto() {
        let payload = "test transaction";
        let encrypted_message = EncryptedMessage::from_payload(&payload);

        assert!(encrypted_message.is_err());
    }

    #[test]
    fn test_should_return_encrypted_message_dto() {
        let encrypted_message = SENDER
            .encrypt_message("test transaction", RECIPIENT.public_account)
            .unwrap();
        let plain_message = RECIPIENT
            .decrypt_message(&encrypted_message, SENDER.public_account)
            .unwrap();

        assert_eq!(plain_message.payload, "test transaction");
    }

    #[test]
    fn test_should_decrypt_message_from_raw_encrypted_message_payload() {
        let encrypted_message = SENDER
            .encrypt_message("Testing simple transfer", RECIPIENT.public_account)
            .unwrap();
        let encrypted_message_from_payload =
            EncryptedMessage::from_payload(&encrypted_message.payload).unwrap();
        let plain_message = RECIPIENT
            .decrypt_message(&encrypted_message_from_payload, SENDER.public_account)
            .unwrap();

        assert_eq!(plain_message.payload, "Testing simple transfer");
    }
}
