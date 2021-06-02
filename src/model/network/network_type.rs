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
    /// The public test network identifier. Decimal value = 152.
    TEST_NET = 0x98,

    /// The public main net network identifier. Decimal value = 104.
    MAIN_NET = 0x68,

    /// Mijin private test network identifier. Decimal value = 144.
    MIJIN_TEST = 0x90,

    /// Mijin private network identifier. Decimal value = 96.
    MIJIN = 0x60,

    UnknownNetworkType,
}

impl NetworkType {
    pub const PREFIX_TEST_NET: char = 'T';
    pub const PREFIX_MAIN_NET: char = 'N';
    pub const PREFIX_MIJIN_TEST: char = 'S';
    pub const PREFIX_MIJIN: char = 'M';

    pub fn value(self) -> u8 {
        self as u8
    }

    pub fn prefix(&self) -> char {
        use NetworkType::*;
        match *self {
            TEST_NET => Self::PREFIX_TEST_NET,
            MAIN_NET => Self::PREFIX_MAIN_NET,
            MIJIN_TEST => Self::PREFIX_MIJIN_TEST,
            MIJIN => Self::PREFIX_MIJIN,
            _ => '_',
        }
    }
}

impl fmt::Display for NetworkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NetworkType::*;
        match *self {
            MAIN_NET => write!(f, "MainNet"),
            TEST_NET => write!(f, "TestNet"),
            MIJIN => write!(f, "Mijin"),
            MIJIN_TEST => write!(f, "MijinTest"),
            UnknownNetworkType => write!(f, "UnknownNetworkType"),
        }
    }
}

// /// `NetworkType` implies From Into
// ///
// impl From<NetworkType> for u8 {
//     fn from(t: NetworkType) -> Self {
//         t as u8
//     }
// }
//
/// Returns a 'NetworkType' for the given u8 value.
///
/// Throws an Err UnknownNetworkType when the type is unknown.
impl std::convert::TryFrom<u8> for NetworkType {
    type Error = anyhow::Error;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        use NetworkType::*;
        match v {
            x if x == MAIN_NET as u8 => Ok(MAIN_NET),
            x if x == TEST_NET as u8 => Ok(TEST_NET),
            x if x == MIJIN as u8 => Ok(MIJIN),
            x if x == MIJIN_TEST as u8 => Ok(MIJIN_TEST),

            _ => Err(anyhow::anyhow!(UnknownNetworkType.to_string())),
        }
    }
}

impl From<char> for NetworkType {
    fn from(ch: char) -> Self {
        use NetworkType::*;
        match ch {
            Self::PREFIX_TEST_NET => TEST_NET,
            Self::PREFIX_MAIN_NET => MAIN_NET,
            Self::PREFIX_MIJIN_TEST => MIJIN_TEST,
            Self::PREFIX_MIJIN => MIJIN,
            _ => UnknownNetworkType,
        }
    }
}

/// Creates `NetworkType` with the default parameters.
impl Default for NetworkType {
    fn default() -> Self {
        NetworkType::TEST_NET
    }
}
