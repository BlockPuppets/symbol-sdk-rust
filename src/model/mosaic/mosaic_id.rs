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
use std::any::Any;

use anyhow::{ensure, Result};

use crate::account::Address;
use crate::Uint64;

use super::{generate_mosaic_id, MosaicNonce, UnresolvedMosaicId};

/// The `MosaicId` structure describes mosaic id.
///
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MosaicId(Uint64);

impl MosaicId {
    /// The length of the `MosaicId` in bytes.
    ///
    const LENGTH_IN_BYTES: usize = 8;

    /// The length of the `MosaicId` in hex string.
    ///
    pub const LENGTH_IN_HEX: usize = Self::LENGTH_IN_BYTES * 2;

    /// Creates a new `MosaicId` from a hex string.
    ///
    pub fn from_hex(hex: &str) -> Result<Self> {
        ensure!(
            hex.len() == Self::LENGTH_IN_HEX,
            "Invalid size for MosaicId hex"
        );

        Ok(Self(Uint64::try_from(hex)?))
    }

    /// Create a `MosaicId` for given `MosaicNonce` MosaicNonce and owner `Address`.
    ///
    pub fn create_from_nonce(nonce: MosaicNonce, owner_address: Address) -> Self {
        generate_mosaic_id(nonce, owner_address)
    }
}

#[typetag::serde]
impl UnresolvedMosaicId for MosaicId {
    fn to_uint64(&self) -> Uint64 {
        self.0
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

impl fmt::Display for MosaicId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

/// Creates a `MosaicId` from the given Uint64 type.
///
impl From<Uint64> for MosaicId {
    fn from(e: Uint64) -> Self {
        MosaicId(e)
    }
}

/// Creates a `MosaicId` from the given u64.
///
impl From<u64> for MosaicId {
    fn from(e: u64) -> Self {
        MosaicId(Uint64::from(e))
    }
}

/// Creates a `MosaicId` from the given low and high bits.
///
impl From<[u32; 2]> for MosaicId {
    fn from(lo_hi: [u32; 2]) -> Self {
        Self(Uint64::from_bits(lo_hi[0], lo_hi[1]))
    }
}

impl Deref for MosaicId {
    type Target = Uint64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::account::Address;
    use crate::mosaic::{MosaicId, MosaicNonce};
    use crate::network::NetworkType;

    const PUBLIC_KEY: &str = "b4f12e7c9f6946091e2cb8b6d3a12b50d17ccbbf646386ea27ce2946a7423dcf";

    #[test]
    fn test_should_be_created_from_hex() {
        let mosaic_id = MosaicId::from_hex("85BBEA6CC462B244").unwrap();
        assert_eq!(mosaic_id.to_dto(), [3294802500, 2243684972]);
    }

    #[test]
    #[should_panic(expected = "Invalid size for MosaicId hex")]
    fn test_should_return_panic_invalid_size() {
        MosaicId::from_hex("85BBEA6CC462B24499").unwrap();
    }

    #[test]
    fn test_should_create_given_nonce_and_owner() {
        let owner = Address::from_public_key(PUBLIC_KEY, NetworkType::PrivateTest).unwrap();
        let nonce = MosaicNonce::from(0);

        let mosaic_id = MosaicId::create_from_nonce(nonce, owner);
        assert_eq!(mosaic_id.to_dto(), [3012716716, 1712914778]);
    }

    #[test]
    fn test_should_create_twice_the_same_given_nonce_and_owner() {
        let owner = Address::from_public_key(PUBLIC_KEY, NetworkType::PrivateTest).unwrap();
        let nonce = MosaicNonce::from(0);

        let mosaic_id_one = MosaicId::create_from_nonce(nonce, owner);
        let mosaic_id_two = MosaicId::create_from_nonce(nonce, owner);
        assert_eq!(mosaic_id_one, mosaic_id_two);
    }
}
