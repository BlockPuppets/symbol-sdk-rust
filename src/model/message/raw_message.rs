/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::message::{Message, MessageType};

/// The `RawMessage` that doesn't assume any prefix.
///
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RawMessage {
    /// Message type
    pub r#type: MessageType,
    /// Message payload
    pub payload: String,
}

impl RawMessage {
    /// Create a new `RawMessage`.
    ///
    pub fn new(payload: &[u8]) -> Self {
        Self {
            r#type: MessageType::RawMessageType,
            payload: hex::encode(payload),
        }
    }
}

#[typetag::serde]
impl Message for RawMessage {
    fn message_type(&self) -> MessageType {
        self.r#type
    }
    fn payload(&self) -> String {
        self.payload.to_owned()
    }
}

impl Default for RawMessage {
    fn default() -> Self {
        Self { r#type: MessageType::RawMessageType, payload: "".to_string() }
    }
}
