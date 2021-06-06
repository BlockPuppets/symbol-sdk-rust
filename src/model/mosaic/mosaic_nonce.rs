/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::{ random_bytes};
use anyhow::{ensure, Result};
use fixed_hash::rustc_hex::ToHex;
use hex::FromHex;
use std::fmt;
use std::ops::Deref;

/// The `MosaicNonce` struct.
///
#[derive(Debug, Clone, Deserialize, Serialize, Copy, PartialEq)]
pub struct MosaicNonce([u8; MosaicNonce::LENGTH]);

impl MosaicNonce {
    /// The length of the `MosaicNonce` in bytes.
    ///
    const LENGTH: usize = 4;

    /// The length of the `MosaicNonce` in hex string.
    ///
    pub const LENGTH_IN_HEX: usize = Self::LENGTH * 2;

    /// Creates a new `MosaicNonce` from fixed_bytes.
    ///
    pub fn new(nonce: [u8; Self::LENGTH]) -> MosaicNonce {
        MosaicNonce(nonce)
    }

    /// Creates a random `MosaicNonce`.
    ///
    pub fn random() -> MosaicNonce {
        let nonce = random_bytes::<4>();
        MosaicNonce(nonce)
    }

    /// Creates a new `MosaicNonce` from a hexadecimal string.
    ///
    pub fn from_hex(hex: &str) -> Result<MosaicNonce> {
        ensure!(
            hex.len() == Self::LENGTH_IN_HEX,
            format!(
                "Invalid hex size for nonce, should be {} bytes but received {}'",
                Self::LENGTH_IN_HEX,
                hex.len()
            )
        );

        let bytes = <[u8; Self::LENGTH]>::from_hex(hex)?;

        Ok(MosaicNonce(bytes))
    }

    /// The `MosaicNonce` as number
    ///
    pub fn to_dto(&self) -> u32 {
        u32::from_le_bytes(self.0)
    }

    /// The `MosaicNonce` as number
    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }
}

impl From<[u8; 4]> for MosaicNonce {
    fn from(bytes: [u8; 4]) -> Self {
        MosaicNonce::new(bytes)
    }
}

impl From<u32> for MosaicNonce {
    fn from(num: u32) -> Self {
        MosaicNonce::new(num.to_le_bytes())
    }
}

impl Deref for MosaicNonce {
    type Target = [u8; Self::LENGTH];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for MosaicNonce {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.to_dto())
    }
}

#[cfg(test)]
mod tests {
    use crate::mosaic::MosaicNonce;

    #[test]
    fn test_should_be_created_from_fixed_bytes() {
        let nonce = MosaicNonce::from([1u8, 2, 3, 4]);
        assert_eq!(nonce.to_dto(), 67305985);
        assert_eq!(*nonce, [1, 2, 3, 4]);
    }

    #[test]
    fn test_should_create_random() {
        let nonce = MosaicNonce::random();
        assert!(!(*nonce).is_empty());
    }

    #[test]
    fn test_should_create_random_twice_not_the_same() {
        let nonce_one = MosaicNonce::random();
        let nonce_two = MosaicNonce::random();
        assert_ne!(nonce_one, nonce_two);
    }

    #[test]
    fn test_should_create_from_hex_str() {
        let nonce = MosaicNonce::from_hex("00000000").unwrap();
        assert_eq!(nonce.to_dto(), 0);
        assert_eq!(*nonce, [0u8, 0, 0, 0]);
    }

    #[test]
    #[should_panic(expected = "Invalid hex size for nonce, should be 8 bytes but received 12")]
    fn test_should_create_from_hex_should_panic() {
        MosaicNonce::from_hex("111100000000").unwrap();
    }

    #[test]
    fn test_should_create_from_nonce_hex() {
        let nonce = MosaicNonce::from_hex("FFFFFFC8").unwrap();
        assert_eq!(nonce.to_hex().to_uppercase(), "FFFFFFC8");
        assert_eq!(nonce.to_dto(), 3372220415);

        let nonce2 = MosaicNonce::from(nonce.to_dto());
        assert_eq!(nonce2.to_hex().to_uppercase(), "FFFFFFC8");
    }

    #[test]
    fn test_should_create_from_hex_with_u32_input() {
        let hex = format!("{:X}", 1845149376_u32);
        let nonce = MosaicNonce::from_hex(&hex).unwrap();
        assert_eq!(*nonce, [109u8, 250, 190, 192]);
    }

    #[test]
    fn test_should_return_string_value() {
        let nonce = MosaicNonce::from([0u8, 0, 0, 0]);
        assert_eq!(nonce.to_hex(), "00000000");

        let nonce = MosaicNonce::from([1u8, 2, 3, 4]);
        assert_eq!(nonce.to_hex(), "01020304");
    }
}
