/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

use anyhow::{ensure, Result};

use crate::core::format::alias_to_recipient;
use crate::network::NetworkType;
use crate::{Uint64, UnresolvedMosaicId};

use super::namespace_id;
use crate::account::UnresolvedAddress;
use std::any::Any;

/// The `NamespaceId` structure describes mosaic id.
///
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct NamespaceId {
    pub id: Uint64,
    #[serde(skip_serializing, skip_deserializing)]
    pub full_name: Option<String>,
}

impl NamespaceId {
    /// The length of the `NamespaceId` in bytes.
    ///
    const LENGTH_IN_BYTES: usize = 8;

    /// The length of the `NamespaceId` in hex string.
    ///
    pub const LENGTH_IN_HEX: usize = Self::LENGTH_IN_BYTES * 2;

    pub fn create_from_name(name: &str) -> Result<NamespaceId> {
        let id = namespace_id(name)?;

        Ok(Self {
            id,
            full_name: Some(name.to_owned()),
        })
    }

    /// Create a NamespaceId from its encoded hexadecimal notation.
    ///
    pub fn from_hex(hex: &str) -> Result<Self> {
        ensure!(
            hex.len() == Self::LENGTH_IN_HEX,
            "Invalid size for NamespaceId hex"
        );

        Ok(Self::from(Uint64::try_from(hex)?))
    }

    /// Encoded unresolved address.
    ///
    pub fn encode_unresolved_address(&self, network_type: NetworkType) -> Vec<u8> {
        alias_to_recipient(self.id, network_type)
    }
}

#[typetag::serde]
impl UnresolvedMosaicId for NamespaceId {
    fn to_uint64(&self) -> Uint64 {
        self.id
    }
    fn box_clone(&self) -> Box<dyn UnresolvedMosaicId + 'static> {
        Box::new((*self).clone())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

#[typetag::serde]
impl UnresolvedAddress for NamespaceId {
    fn recipient_to_string(&self) -> String {
        self.id.to_hex()
    }

    fn unresolved_address_to_bytes(&self, network_type: NetworkType) -> Vec<u8> {
        self.encode_unresolved_address(network_type)
    }

    fn box_clone(&self) -> Box<dyn UnresolvedAddress + 'static> {
        Box::new((*self).clone())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

/// Creates a `NamespaceId` from the given Uint64 type.
///
impl From<Uint64> for NamespaceId {
    fn from(u: Uint64) -> Self {
        NamespaceId {
            id: u,
            full_name: None,
        }
    }
}

/// Creates a `NamespaceId` from the given u64.
///
impl From<u64> for NamespaceId {
    fn from(e: u64) -> Self {
        NamespaceId {
            id: Uint64::from(e),
            full_name: None,
        }
    }
}

/// Creates a `NamespaceId` from the given low and high bits.
///
impl From<[u32; 2]> for NamespaceId {
    fn from(lo_hi: [u32; 2]) -> Self {
        Self {
            id: Uint64::from_bits(lo_hi[0], lo_hi[1]),
            full_name: None,
        }
    }
}

impl Deref for NamespaceId {
    type Target = Uint64;
    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl fmt::Display for NamespaceId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ \"id\": \"{}\", \"full_name\": \"{}\" }}",
            self.id.to_hex(),
            self.full_name.as_ref().unwrap_or(&"".to_string())
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::namespace::NamespaceId;

    #[test]
    fn test_should_be_created_from_root_namespace_name() {
        let id = NamespaceId::create_from_name("nem").unwrap();
        assert_eq!(id.to_dto(), [929036875, 2226345261]);
        assert_eq!(id.full_name, Some("nem".to_string()));
    }

    #[test]
    fn test_should_be_created_from_sub_namespace_name() {
        let id = NamespaceId::create_from_name("nem.subnem").unwrap();
        assert_eq!(id.to_dto(), [373240754, 3827892399]);
        assert_eq!(id.full_name, Some("nem.subnem".to_string()));
    }

    #[test]
    fn test_should_be_created_from_id() {
        let id = NamespaceId::from([3646934825, 3576016193]);
        assert_eq!(id.to_dto(), [3646934825, 3576016193]);
        assert_eq!(id.full_name, None);
    }
}