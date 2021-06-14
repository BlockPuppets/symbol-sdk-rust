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
use std::num::ParseIntError;
use std::ops::Deref;

use byteorder::{BigEndian, ReadBytesExt};
use hex::{FromHex, FromHexError};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
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
        [self.to_lower(), self.to_higher()]
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
        *self.deref()
    }

    /// Returns a u64 to the value.
    ///
    #[inline]
    pub fn to_hex(&self) -> String {
        format!("{:X}", self)
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

impl From<[u32; 2]> for Uint64 {
    #[inline]
    fn from(slice: [u32; 2]) -> Self {
        Self::from_bits(slice[0], slice[1])
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

    /// Creates a `Uint64` instance from the given hex string.
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
    type Err = ParseIntError;

    /// Parses a numeric string into a UInt64.
    ///
    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse::<u64>()?))
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
        Self(*self + *other)
    }
}

/// The subtraction operator `-` for `Uint64`.
///
impl std::ops::Sub for Uint64 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(*self - *other)
    }
}

/// The bitwise AND operator `&` for `Uint64`.
///
impl std::ops::BitAnd for Uint64 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Uint64(*self & *rhs)
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

impl std::fmt::Display for Uint64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::write!(f, "{}", self.deref())
    }
}

pub trait AsUint64 {
    fn as_uint64(&self) -> Uint64;

    fn to_dto(&self) -> [u32; 2] {
        self.as_uint64().to_dto()
    }

    /// u64 higher part.
    ///
    #[inline]
    fn to_higher(&self) -> u32 {
        (*self.as_uint64() >> 32) as u32
    }

    /// u64 lower part.
    ///
    #[inline]
    fn to_lower(&self) -> u32 {
        *self.as_uint64() as u32
    }
}

impl AsUint64 for u64 {
    fn as_uint64(&self) -> Uint64 {
        Uint64::from(*self)
    }
}

impl From<Uint64> for u64 {
    fn from(value: Uint64) -> Self {
        *value
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::str::FromStr;

    use crate::{AsUint64, Uint64};

    struct TestVector {
        str: &'static str,
        value: [u32; 2],
    }

    static HEX_TEST_VECTORS: &[TestVector] = &[
        TestVector {
            str: "0000000000000000",
            value: [0, 0],
        }, // "0"
        TestVector {
            str: "000000000000A1B2",
            value: [0xa1b2, 0],
        }, // "(0, 8)"
        TestVector {
            str: "0000000012345678",
            value: [0x12345678, 0],
        }, // "8"
        TestVector {
            str: "0000ABCD12345678",
            value: [0x12345678, 0xabcd],
        }, //"(8, 16)"
        TestVector {
            str: "1234567890ABCDEF",
            value: [0x90abcdef, 0x12345678],
        }, // "16"
        TestVector {
            str: "FFFFFFFFFFFFFFFF",
            value: [0xffffffff, 0xffffffff],
        }, // "16 (max value)"
    ];

    #[test]
    fn test_max_value() {
        assert_eq!(Uint64::MAX, 18446744073709551615);
    }

    #[test]
    fn test_should_create_from_u32slice() {
        let slice = [0u32, 0];
        let uint64 = Uint64::from(slice);
        assert_eq!(uint64.to_lower(), slice[0]);
        assert_eq!(uint64.to_higher(), slice[1]);
    }

    #[test]
    fn test_should_create_from_u64() {
        let uint64 = Uint64::from(51110867862);
        assert_eq!(uint64.to_lower(), 3866227606);
        assert_eq!(uint64.to_higher(), 11);
    }

    #[test]
    fn test_should_defer_u64() {
        let uint64 = Uint64::from([3866227606, 11]);
        assert_eq!(*uint64, 51110867862);
    }

    #[test]
    fn test_should_return_true_if_the_inside_values_are_the_same() {
        let value = Uint64::from([12, 12]);
        let other = Uint64::from([12, 12]);

        assert_eq!(value, other);
    }

    #[test]
    fn test_should_return_true_if_the_inside_values_are_the_same_but_different_order() {
        let value = Uint64::from([12, 23]);
        let other = Uint64::from([23, 12]);

        assert_ne!(value, other);
    }

    #[cfg(test)]
    mod test_from_hex {
        use super::*;

        #[test]
        fn test_should_create_from_hex_str() {
            for test_case in HEX_TEST_VECTORS {
                let value = Uint64::try_from(test_case.str).unwrap();
                assert_eq!(value.to_dto(), test_case.value);
            }
        }

        #[test]
        #[should_panic(expected = "InvalidHexCharacter { c: 'G', index: 13 }")]
        fn test_cannot_parse_hex_str_with_invalid_characters() {
            Uint64::try_from("0000000012345G78").unwrap();
        }

        #[test]
        fn test_cannot_parse_hex_str_with_invalid_str_len() {
            let error = hex::FromHexError::InvalidStringLength;
            assert_eq!(Uint64::try_from("").unwrap_err(), error);
            assert_eq!(Uint64::try_from("1234567890ABCDEF12").unwrap_err(), error);
            assert_eq!(Uint64::try_from("ABCDEF12").unwrap_err(), error);
        }

        #[test]
        fn test_cannot_parse_hex_str_with_odd_len() {
            let error = hex::FromHexError::OddLength;
            assert_eq!(Uint64::try_from("1").unwrap_err(), error);
            assert_eq!(Uint64::try_from("123").unwrap_err(), error);
            assert_eq!(Uint64::try_from("ABCDE").unwrap_err(), error);
        }
    }

    #[cfg(test)]
    mod test_from_str {
        use super::*;

        #[test]
        fn test_should_create_from_str() {
            let value = "1000";
            let uint64 = Uint64::from_str(value).unwrap();
            assert_eq!(uint64.to_lower(), 1000);

            let value = "51110867862";
            let uint64 = Uint64::from_str(value).unwrap();
            assert_eq!(uint64.to_lower(), 3866227606);
            assert_eq!(uint64.to_higher(), 11);
        }

        #[test]
        fn test_should_return_numeric_string() {
            let value = "1000";
            let uint64 = Uint64::from_str(value).unwrap();
            assert_eq!(uint64.to_string(), value);
        }

        #[test]
        #[should_panic(expected = "InvalidDigit")]
        fn test_should_panic_when_creating_from_invalid_numeric_str() {
            Uint64::from_str("ABC12345678").unwrap();
        }
    }

    #[cfg(test)]
    mod test_operation {
        use super::*;

        #[test]
        fn test_should_return_added_value() {
            let value = Uint64::from(0);
            let other = Uint64::from(0);
            let result = value + other;
            assert_eq!(*result, 0);

            let value = Uint64::from(100);
            let other = Uint64::from(1);
            let result = value + other;
            assert_eq!(*result, 101);
        }

        #[test]
        fn test_should_return_substract_value() {
            let value = Uint64::from(100);
            let other = Uint64::from(1);
            let result = value - other;
            assert_eq!(*result, 99);

            let value = Uint64::from(1);
            let other = Uint64::from(1);
            let result = value - other;
            assert_eq!(*result, 0);

            let value = Uint64::from(100);
            let other = Uint64::from(1);
            let result = other.checked_sub(*value);
            assert!(result.is_none());
        }
    }
}
