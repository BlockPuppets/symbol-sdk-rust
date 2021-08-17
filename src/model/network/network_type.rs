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

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[repr(u8)]
pub enum NetworkType {
    /// The Public test net network identifier.
    /// Decimal value = 152
    /// Hex value = 0x98
    ///
    TestNet = 0x98,

    /// The public main net network identifier.
    /// Decimal value = 104
    /// Hex value = 0x68
    ///
    MainNet = 0x68,

    /// The Private net network identifier.
    /// Decimal value = 120
    /// Hex value = 0x78
    ///
    Private = 0x78,

    /// The Private test net network identifier.
    /// Decimal value = 168
    /// Hex value = 0xa8
    ///
    PrivateTest = 0xa8,

    /// Mijin private test network identifier.
    /// Decimal value = 144
    /// Hex value = 0x90
    ///
    MijinTest = 0x90,

    /// Mijin private network identifier.
    /// Decimal value = 96
    /// Hex value = 0x60
    ///
    Mijin = 0x60,
}

impl NetworkType {
    pub const PREFIX_TEST_NET: char = 'T';
    pub const PREFIX_MAIN_NET: char = 'N';
    pub const PREFIX_PRIVATE_TEST: char = 'V';
    pub const PREFIX_PRIVATE: char = 'P';
    pub const PREFIX_MIJIN_TEST: char = 'S';
    pub const PREFIX_MIJIN: char = 'M';

    const UNKNOWN_NETWORK_TYPE: &'static str = "Address Network unsupported";

    pub fn value(self) -> u8 {
        self as u8
    }

    pub fn prefix(&self) -> char {
        use NetworkType::*;
        match *self {
            TestNet => Self::PREFIX_TEST_NET,
            MainNet => Self::PREFIX_MAIN_NET,
            PrivateTest => Self::PREFIX_PRIVATE_TEST,
            Private => Self::PREFIX_PRIVATE,
            MijinTest => Self::PREFIX_MIJIN_TEST,
            Mijin => Self::PREFIX_MIJIN,
        }
    }

    pub fn to_bytes(&self) -> [u8; 1] {
        self.value().to_le_bytes()
    }

    /// Create Builder object
    pub fn to_builder(&self) -> buffer::network_type_dto::NetworkTypeDto {
        buffer::network_type_dto::NetworkTypeDto::from_binary(&self.to_bytes())
    }
}

impl fmt::Display for NetworkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

/// Returns a 'NetworkType' for the given u8 value.
///
/// Throws an Err UNKNOWN_NETWORK_TYPE when the type is unknown.
impl TryFrom<u8> for NetworkType {
    type Error = anyhow::Error;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        use NetworkType::*;
        match v {
            x if x == MainNet as u8 => Ok(MainNet),
            x if x == TestNet as u8 => Ok(TestNet),
            x if x == Private as u8 => Ok(Private),
            x if x == PrivateTest as u8 => Ok(PrivateTest),
            x if x == Mijin as u8 => Ok(Mijin),
            x if x == MijinTest as u8 => Ok(MijinTest),
            _ => Err(anyhow::anyhow!(Self::UNKNOWN_NETWORK_TYPE)),
        }
    }
}

/// Returns a 'NetworkType' for the given char value.
///
/// Throws an Err UNKNOWN_NETWORK_TYPE when the type is unknown.
impl TryFrom<char> for NetworkType {
    type Error = anyhow::Error;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        use NetworkType::*;
        match ch {
            Self::PREFIX_TEST_NET => Ok(TestNet),
            Self::PREFIX_MAIN_NET => Ok(MainNet),
            Self::PREFIX_PRIVATE_TEST => Ok(PrivateTest),
            Self::PREFIX_PRIVATE => Ok(Private),
            Self::PREFIX_MIJIN_TEST => Ok(MijinTest),
            Self::PREFIX_MIJIN => Ok(Mijin),
            _ => Err(anyhow::anyhow!(Self::UNKNOWN_NETWORK_TYPE)),
        }
    }
}

/// Creates `NetworkType` with the default parameters.
///
impl Default for NetworkType {
    fn default() -> Self {
        NetworkType::TestNet
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
        assert_eq!(NetworkType::MainNet as u8, 0x68);
        assert_eq!(NetworkType::MainNet as u8, 104);
    }

    #[test]
    fn test_test_net_is_0x98() {
        assert_eq!(NetworkType::TestNet as u8, 0x98);
        assert_eq!(NetworkType::TestNet as u8, 152);
    }

    #[test]
    fn test_private_test_is_0xa8() {
        assert_eq!(NetworkType::PrivateTest as u8, 0xa8);
        assert_eq!(NetworkType::PrivateTest as u8, 168);
    }

    #[test]
    fn test_private_is_0x78() {
        assert_eq!(NetworkType::Private as u8, 0x78);
        assert_eq!(NetworkType::Private as u8, 120);
    }

    #[test]
    fn test_mijin_is_0x60() {
        assert_eq!(NetworkType::Mijin as u8, 0x60);
        assert_eq!(NetworkType::Mijin as u8, 96);
    }

    #[test]
    fn test_mijin_test_is_0x90() {
        assert_eq!(NetworkType::MijinTest as u8, 0x90);
        assert_eq!(NetworkType::MijinTest as u8, 144);
    }
}
