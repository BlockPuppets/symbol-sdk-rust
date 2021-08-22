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

/// The Message type. Supported supply types are:
///
/// * -1 - RawMessage (no type appended).
/// * 0 - Plain text or unencrypted message.
/// * 1 - Secured text or encrypted message.
/// * 254 - Persistent harvesting delegation message.
///
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Copy)]
#[repr(i16)]
pub enum MessageType {
    /// RawMessage.
    #[serde(rename = "-1")]
    RawMessageType = -1,
    /// Plain text or unencrypted message.
    #[serde(rename = "0")]
    PlainMessageType = 0x00,
    /// Secured text or encrypted message.
    #[serde(rename = "1")]
    SecureMessageType = 0x01,
    /// Persistent harvesting delegation message.
    #[serde(rename = "254")]
    PersistentHarvestingDelegationMessageType = 0xfe,

    UnknownMessageType = 10,
}

impl MessageType {
    pub fn value(self) -> i16 {
        self as i16
    }
    pub fn to_bytes(&self) -> [u8; 1] {
        (self.value() as u8).to_le_bytes()
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MessageType::*;
        match *self {
            RawMessageType => write!(f, "RawMessage"),
            PlainMessageType => write!(f, "PlainMessageType"),
            SecureMessageType => write!(f, "SecureMessageType"),
            PersistentHarvestingDelegationMessageType => {
                write!(f, "PersistentHarvestingDelegationMessage")
            }
            UnknownMessageType => write!(f, "UnknownMessageType"),
        }
    }
}

/// Returns a 'MessageType' for the given i8 value.
///
/// Throws an UnknownMessageType when the type is unknown.
impl From<i16> for MessageType {
    fn from(num: i16) -> Self {
        match num {
            -1 => MessageType::RawMessageType,
            0x00 => MessageType::PlainMessageType,
            0x01 => MessageType::SecureMessageType,
            0xfe => MessageType::PersistentHarvestingDelegationMessageType,
            _ => MessageType::UnknownMessageType,
        }
    }
}

/// Creates `MessageType` with the default parameters.
impl Default for MessageType {
    fn default() -> Self {
        Self::PlainMessageType
    }
}

#[cfg(test)]
mod tests {
    use crate::message::MessageType;

    #[test]
    fn test_message_type_is_negative() {
        assert_eq!(MessageType::RawMessageType as i16, -1);
    }

    #[test]
    fn test_plain_message_type_is_0x00() {
        assert_eq!(MessageType::PlainMessageType as i16, 0x00);
        assert_eq!(MessageType::PlainMessageType as i16, 0);
    }

    #[test]
    fn test_secure_message_type_is_0x01() {
        assert_eq!(MessageType::SecureMessageType as i16, 0x01);
        assert_eq!(MessageType::SecureMessageType as i16, 1);
    }

    #[test]
    fn test_persistent_harvesting_delegation_message_type_is_0xfe() {
        assert_eq!(
            MessageType::PersistentHarvestingDelegationMessageType as i16,
            0xfe
        );
        assert_eq!(
            MessageType::PersistentHarvestingDelegationMessageType as i16,
            254
        );
    }
}
