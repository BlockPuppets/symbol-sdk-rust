use std::str::FromStr;

use anyhow::{ensure, Result};

use crate::{hex_to_utf8, is_hex};
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

        let signer_private_key = crypto::PrivateKey::from_str(signer_private_key)?;
        let recipient_public_key = crypto::PublicKey::from_str(recipient_public_key)?;

        let encrypt_message = signer_private_key.encrypt_message::<crypto::CryptoSym>(
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
        let payload = hex_to_utf8(payload_hex);
        Ok(Self {
            r#type: MessageType::SecureMessageType,
            payload,
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
        let recipient_private_key = crypto::PrivateKey::from_str(recipient_private_key)?;
        let signer_public_key = crypto::PublicKey::from_str(signer_public_key)?;

        let decrypted_message = recipient_private_key.decrypt_message::<crypto::CryptoSym>(
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
