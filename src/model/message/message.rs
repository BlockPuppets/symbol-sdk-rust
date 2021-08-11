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

use crate::message::{
    MessageType, PersistentHarvestingDelegationMessage, PlainMessage, RawMessage,
    PERSISTENT_DELEGATION_UNLOCK,
};
use crate::{hex_decode, is_hex};

/// An abstract message trait that serves as the base of all message types.
///
#[typetag::serde]
pub trait Message: Sync + Send
where
    Self: fmt::Debug,
{
    fn message_type(&self) -> MessageType;

    fn payload_to_vec(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let payload_str = self.payload();
        if is_hex(&payload_str) {
            buf.extend(hex::decode(&payload_str).unwrap())
        } else {
            buf.extend(payload_str.as_bytes())
        };

        buf
    }

    fn payload(&self) -> String;

    fn to_dto(&self) -> String {
        use MessageType::*;

        if self.payload().is_empty() {
            return String::new();
        };
        match self.message_type() {
            RawMessageType | PersistentHarvestingDelegationMessageType => self.payload(),
            PlainMessageType | SecureMessageType => {
                let dto = hex::encode((self.message_type().value() as u8).to_be_bytes());
                dto + &hex::encode(&self.payload_to_vec())
            }
            _ => String::new(),
        }
        .to_uppercase()
    }

    fn box_clone(&self) -> Box<dyn Message>;

    fn to_vec(&self) -> Vec<u8> {
        if self.payload().is_empty() {
            return vec![];
        };
        let mut buf = Vec::new();
        buf.extend(self.message_type().to_bytes().to_vec());
        buf.extend(self.payload_to_vec());
        buf
    }
}

impl Clone for Box<dyn Message + 'static> {
    fn clone(&self) -> Box<dyn Message + 'static> {
        self.box_clone()
    }
}

impl<'a> PartialEq for &'a dyn Message {
    fn eq(&self, other: &Self) -> bool {
        self.to_vec() == other.to_vec()
    }
}

impl<'a> PartialEq for Box<dyn Message + 'static> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

/// It creates a message from the hex payload.
///
pub(crate) fn create_message_from_hex<S: AsRef<str>>(payload: S) -> Result<Box<dyn Message>> {
    if payload.as_ref().is_empty() {
        return Ok(Box::new(RawMessage::default()));
    }

    let payload_uppercase = payload.as_ref().to_uppercase();

    if payload_uppercase.len() == PersistentHarvestingDelegationMessage::HEX_PAYLOAD_SIZE
        && payload_uppercase.starts_with(PERSISTENT_DELEGATION_UNLOCK)
    {
        return Ok(Box::new(
            PersistentHarvestingDelegationMessage::from_payload(&payload_uppercase)?,
        ));
    }

    let payload_vec = hex_decode(&payload_uppercase);

    let message_type = MessageType::from(i16::from(payload_vec[0]));
    let payload_without_prefix = hex::encode(&payload_vec[1..]);

    match message_type {
        MessageType::PlainMessageType => Ok(Box::new(PlainMessage::from_payload(
            &payload_without_prefix,
        )?)),
        MessageType::SecureMessageType => unimplemented!(),
        _ => Ok(Box::new(RawMessage::new(&payload_vec[1..]))),
    }
}

/// It creates a message from the byte array payload.
///
pub(crate) fn create_message_from_buffer(payload: &[u8]) -> Option<Box<dyn Message>> {
    if payload.is_empty() {
        return None;
    }

    if let Ok(msg) = create_message_from_hex(hex::encode(payload)) {
        Some(msg)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::message::{EncryptedMessage, Message, MessageType, PlainMessage};
    use crate::utf8_to_hex;

    #[test]
    fn test_should_create_an_plain_message_dto_struct() {
        let message = PlainMessage::create("test");
        assert_eq!(
            message.to_dto(),
            format!("00{}", utf8_to_hex("test").to_uppercase())
        );
    }

    #[test]
    fn test_should_create_an_encrypted_message_dto_struct() {
        let message = EncryptedMessage {
            r#type: MessageType::SecureMessageType,
            payload: "test".to_string(),
        };
        assert_eq!(
            message.to_dto(),
            format!("01{}", utf8_to_hex("test").to_uppercase())
        );
    }
}
