/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[repr(u8)]
pub enum NetworkType {
    /// The Public test net network identifier.
    /// Decimal value = 152.
    ///
    TEST_NET = 0x98,

    /// The public main net network identifier.
    /// Decimal value = 104.
    ///
    MAIN_NET = 0x68,

    /// The Private net network identifier.
    /// Decimal value = 120.
    ///
    PRIVATE = 0x78,

    /// The Private test net network identifier.
    /// Decimal value = 168.
    ///
    PRIVATE_TEST = 0xa8,

    /// Mijin private test network identifier.
    /// Decimal value = 144.
    ///
    MIJIN_TEST = 0x90,

    /// Mijin private network identifier.
    /// Decimal value = 96.
    ///
    MIJIN = 0x60,
}

impl NetworkType {
    pub const PREFIX_TEST_NET: char = 'T';
    pub const PREFIX_MAIN_NET: char = 'N';
    pub const PREFIX_PRIVATE_TEST: char = 'P';
    pub const PREFIX_PRIVATE: char = 'V';
    pub const PREFIX_MIJIN_TEST: char = 'S';
    pub const PREFIX_MIJIN: char = 'M';

    const UNKNOWN_NETWORK_TYPE: &'static str = "Unknown NetworkType";

    pub fn value(self) -> u8 {
        self as u8
    }

    pub fn prefix(&self) -> char {
        use NetworkType::*;
        match *self {
            TEST_NET => Self::PREFIX_TEST_NET,
            MAIN_NET => Self::PREFIX_MAIN_NET,
            PRIVATE_TEST => Self::PREFIX_PRIVATE_TEST,
            PRIVATE => Self::PREFIX_PRIVATE,
            MIJIN_TEST => Self::PREFIX_MIJIN_TEST,
            MIJIN => Self::PREFIX_MIJIN,
        }
    }
}

impl fmt::Display for NetworkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NetworkType::*;
        match *self {
            MAIN_NET => write!(f, "MainNet"),
            TEST_NET => write!(f, "TestNet"),
            PRIVATE => write!(f, "Private"),
            PRIVATE_TEST => write!(f, "PrivateTest"),
            MIJIN => write!(f, "Mijin"),
            MIJIN_TEST => write!(f, "MijinTest"),
        }
    }
}

/// Returns a 'NetworkType' for the given u8 value.
///
/// Throws an Err UNKNOWN_NETWORK_TYPE when the type is unknown.
impl std::convert::TryFrom<u8> for NetworkType {
    type Error = anyhow::Error;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        use NetworkType::*;
        match v {
            x if x == MAIN_NET as u8 => Ok(MAIN_NET),
            x if x == TEST_NET as u8 => Ok(TEST_NET),
            x if x == PRIVATE as u8 => Ok(PRIVATE),
            x if x == PRIVATE_TEST as u8 => Ok(PRIVATE_TEST),
            x if x == MIJIN as u8 => Ok(MIJIN),
            x if x == MIJIN_TEST as u8 => Ok(MIJIN_TEST),
            _ => Err(anyhow::anyhow!(Self::UNKNOWN_NETWORK_TYPE)),
        }
    }
}

/// Returns a 'NetworkType' for the given char value.
///
/// Throws an Err UNKNOWN_NETWORK_TYPE when the type is unknown.
impl std::convert::TryFrom<char> for NetworkType {
    type Error = anyhow::Error;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        use NetworkType::*;
        match ch {
            Self::PREFIX_TEST_NET => Ok(TEST_NET),
            Self::PREFIX_MAIN_NET => Ok(MAIN_NET),
            Self::PREFIX_PRIVATE_TEST => Ok(PRIVATE_TEST),
            Self::PREFIX_PRIVATE => Ok(PRIVATE),
            Self::PREFIX_MIJIN_TEST => Ok(MIJIN_TEST),
            Self::PREFIX_MIJIN => Ok(MIJIN),
            _ => Err(anyhow::anyhow!(Self::UNKNOWN_NETWORK_TYPE)),
        }
    }
}

/// Creates `NetworkType` with the default parameters.
///
impl Default for NetworkType {
    fn default() -> Self {
        NetworkType::TEST_NET
    }
}

impl From<NetworkType> for u8 {
    fn from(value: NetworkType) -> Self {
        value as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::network::NetworkType;

    #[test]
    fn test_main_net_is_0x68() {
        assert_eq!(NetworkType::MAIN_NET as u8, 0x68);
        assert_eq!(NetworkType::MAIN_NET as u8, 104);
    }

    #[test]
    fn test_test_net_is_0x98() {
        assert_eq!(NetworkType::TEST_NET as u8, 0x98);
        assert_eq!(NetworkType::TEST_NET as u8, 152);
    }

    #[test]
    fn test_private_test_is_0xa8() {
        assert_eq!(NetworkType::PRIVATE_TEST as u8, 0xa8);
        assert_eq!(NetworkType::PRIVATE_TEST as u8, 168);
    }

    #[test]
    fn test_private_is_0x78() {
        assert_eq!(NetworkType::PRIVATE as u8, 0x78);
        assert_eq!(NetworkType::PRIVATE as u8, 120);
    }

    #[test]
    fn test_mijin_is_0x60() {
        assert_eq!(NetworkType::MIJIN as u8, 0x60);
        assert_eq!(NetworkType::MIJIN as u8, 96);
    }

    #[test]
    fn test_mijin_test_is_0x90() {
        assert_eq!(NetworkType::MIJIN_TEST as u8, 0x90);
        assert_eq!(NetworkType::MIJIN_TEST as u8, 144);
    }
}
