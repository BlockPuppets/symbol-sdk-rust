/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::convert::TryFrom;
use std::io::{Cursor, Write};
use std::ops::Deref;

use byteorder::{BigEndian, ReadBytesExt};
use hex::{FromHex, FromHexError};

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Uint64(u64);

impl Uint64 {
    /// The smallest value that can be represented by this integer type.
    ///
    pub const MIN: u64 = 0;

    /// The largest value that can be represented by this integer type.
    ///
    /// # Note
    ///
    /// The MAX value of Uint64 is 18446744073709551615.
    ///
    pub const MAX: u64 = !0;

    /// The size of this integer type in bits.
    ///
    const BITS: usize = 64;

    /// The size of this integer type in bytes.
    ///
    const BYTES: usize = Self::BITS / 8;

    /// Creates a `Uint64` from the given low and high bits.
    ///
    #[inline]
    pub fn from_bits(lower: u32, higher: u32) -> Self {
        let mut buf = [0x0; Self::BYTES];
        buf[..4]
            .as_mut()
            .write(&higher.to_be_bytes())
            .expect("higher write error");
        buf[4..]
            .as_mut()
            .write(&lower.to_be_bytes())
            .expect("lower write error");
        Self::from(buf)
    }

    /// Returns the size in bytes.
    ///
    #[inline]
    pub fn len_bytes() -> usize {
        std::mem::size_of::<Self>()
    }

    /// Returns the inner bytes array.
    ///
    #[inline]
    pub fn to_fixed_bytes(self) -> [u8; Self::BYTES] {
        self.to_be_bytes()
    }

    /// Get DTO representation with format: `[lower, higher]`
    ///
    #[inline]
    pub fn to_dto(self) -> [u32; 2] {
        let lower = self.0 as u32;
        let higher = (self.0 >> 32) as u32;
        [lower, higher]
    }

    /// Returns a constant raw pointer to the value.
    ///
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.to_fixed_bytes().as_ptr()
    }

    /// Returns a u64 to the value.
    ///
    #[inline]
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    /// Returns a u64 to the value.
    ///
    #[inline]
    pub fn to_hex(&self) -> String {
        self.to_string()
    }

    /// Creates a new `Uint64` zero-initialized.
    ///
    #[inline]
    pub fn zero() -> Self {
        Self(0)
    }
}

impl From<[u8; 8]> for Uint64 {
    /// Constructs a `Uint64` from the given bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: [u8; Self::BYTES]) -> Self {
        let mut cursor = Cursor::new(bytes[..].as_ref());
        let value = cursor.read_u64::<BigEndian>().unwrap();

        Self(value)
    }
}

impl From<u64> for Uint64 {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a [u8; 8]> for Uint64 {
    /// Constructs `Uint64` from the given reference
    /// to the bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a [u8; 8]) -> Self {
        Self::from(*bytes)
    }
}

impl From<Uint64> for [u8; 8] {
    #[inline]
    fn from(value: Uint64) -> Self {
        value.to_fixed_bytes()
    }
}

impl TryFrom<&str> for Uint64 {
    type Error = FromHexError;

    /// Creates a `Uint64` instance from the given string.
    ///
    /// # Note
    ///
    /// The given input string is interpreted in big endian.
    ///
    /// # Errors
    ///
    /// - When encountering invalid non hex-digits
    /// - Upon empty string input or invalid input length in general
    fn try_from(hex: &str) -> std::result::Result<Self, Self::Error> {
        let bytes = <[u8; Self::BYTES]>::from_hex(hex)?;
        Ok(Self::from(bytes))
    }
}

impl std::str::FromStr for Uint64 {
    type Err = FromHexError;

    /// Parse a value from a hex string.
    ///
    fn from_str(hex: &str) -> std::result::Result<Self, Self::Err> {
        Self::try_from(hex)
    }
}

/// Enable `Deref` coercion `Uint64`.
///
impl Deref for Uint64 {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The addition operator `+` for `Uint64`.
///
impl std::ops::Add for Uint64 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

/// The subtraction operator `-` for `Uint64`.
///
impl std::ops::Sub for Uint64 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

/// The bitwise AND operator `&` for `Uint64`.
///
impl std::ops::BitAnd for Uint64 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Uint64(self.0 & rhs.0)
    }
}

/// Implementing `LowerHex` for `Uint64`.
///
impl std::fmt::LowerHex for Uint64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            std::write!(f, "0x")?;
        }
        for i in &self.to_fixed_bytes() {
            std::write!(f, "{:02x}", i)?;
        }
        Ok(())
    }
}

/// Implementing `UpperHex` for `Uint64`.
///
impl std::fmt::UpperHex for Uint64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            std::write!(f, "0X")?;
        }
        for i in &self.to_fixed_bytes() {
            std::write!(f, "{:02X}", i)?;
        }
        Ok(())
    }
}

impl Default for Uint64 {
    fn default() -> Self {
        Self::zero()
    }
}

impl std::fmt::Debug for Uint64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::write!(f, "{}", self.as_u64())
    }
}

impl std::fmt::Display for Uint64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::write!(f, "{:X}", &self)
    }
}
