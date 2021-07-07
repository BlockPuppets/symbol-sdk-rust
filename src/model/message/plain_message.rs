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

use crate::{hex_to_utf8, is_hex};
use crate::message::{Message, MessageType};

/// The `PlainMessage` struct defines a plain string.
/// When sending it to the network we transform the payload to hex-string.
///
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlainMessage {
    /// Message type
    pub r#type: MessageType,
    /// Message payload, it could be the message plain text.
    pub payload: String,
}

impl PlainMessage {
    /// Create a new `PlainMessage`.
    ///
    pub fn create(message: &str) -> Self {
        Self {
            r#type: MessageType::PlainMessageType,
            payload: message.into(),
        }
    }

    pub fn empty() -> Self {
        Self {
            r#type: MessageType::PlainMessageType,
            payload: String::new(),
        }
    }

    /// It creates the `PlainMessage` from a payload hex without the 00 prefix.
    /// The 00 prefix will be attached to the final payload.
    ///
    pub fn from_payload(payload_hex: &str) -> Result<Self> {
        ensure!(is_hex(payload_hex), "payload_hex it's not hex.");
        let payload = hex_to_utf8(payload_hex);
        Ok(Self {
            r#type: MessageType::PlainMessageType,
            payload,
        })
    }

    /// It creates a `PlainMessage` from the given bytes.
    ///
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        ensure!(!bytes.is_empty(), "bytes must not be empty.");
        let payload = hex::encode(bytes);
        Self::from_payload(&payload)
    }
}

#[typetag::serde]
impl Message for PlainMessage {
    fn message_type(&self) -> MessageType {
        self.r#type
    }
    fn payload(&self) -> String {
        self.payload.to_owned()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::message::{Message, MessageType, PlainMessage};

    #[test]
    fn test_default_plain_message() {
        let plain_message = PlainMessage::default();

        assert_eq!(plain_message.payload, "");
        assert_eq!(plain_message.message_type(), MessageType::PlainMessageType);
    }

    #[test]
    fn test_create_plain_message() {
        let payload = "test-message";

        let plain_message = PlainMessage::create(payload);
        assert_eq!(plain_message.payload(), payload);
        assert_eq!(plain_message.to_dto(), "00746573742D6D657373616765");
    }

    #[test]
    fn test_create_plain_message_with_static_method() {
        let payload = "746573742D6D657373616765";

        let plain_message = PlainMessage::from_payload(payload).unwrap();

        assert_eq!(plain_message.payload, "test-message");
        assert_eq!(plain_message.to_dto(), "00746573742D6D657373616765");
    }
}