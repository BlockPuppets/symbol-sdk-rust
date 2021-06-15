/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::str::FromStr;

use hex::ToHex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;

use crate::core::format::encode_base32;
use crate::account::AddressSym;
use crate::AddressNis1;

pub type GenerationHash = H256;

pub type TransactionHash = H256;

construct_fixed_hash! {
    /// Symbol 256 bit hash type.
    #[derive(Deserialize)]
    pub struct H256(32);
}

impl Serialize for H256 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.encode_hex_upper::<String>())
    }
}

construct_fixed_hash! {
    /// Symbol 512 bit hash type.
    pub struct H512(64);
}

impl<'de> Deserialize<'de> for H512 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        H512::from_str(string.as_ref()).map_err(|e| D::Error::custom(e))
    }
}

impl Serialize for H512 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.encode_hex_upper::<String>())
    }
}

construct_fixed_hash! {
    /// Symbol 192 bit hash type.
    #[derive(Deserialize)]
    pub struct H192(24);
}

impl Serialize for H192 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_base32())
    }
}

impl AddressSchema for H192 {
    type Hash = H192;

    fn as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
    fn size_suffix(&self) -> usize {
        AddressSym::CHECKSUM_SIZE
    }
    fn to_base32(&self) -> String {
        encode_base32(&self.as_bytes(), AddressSym::LENGTH_IN_BASE32)
    }
}

pub trait AddressSchema {
    type Hash;
    fn as_bytes(&self) -> &[u8];
    fn size_suffix(&self) -> usize;
    fn to_base32(&self) -> String;
}

#[cfg(feature = "nis1")]
construct_fixed_hash! {
    /// Nis1 200 bit hash type.
    #[derive(Deserialize)]
    pub struct H200(25);
}

#[cfg(feature = "nis1")]
impl Serialize for H200 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_base32())
    }
}

#[cfg(feature = "nis1")]
impl AddressSchema for H200 {
    type Hash = H200;

    fn as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
    fn size_suffix(&self) -> usize {
        AddressNis1::CHECKSUM_SIZE
    }
    fn to_base32(&self) -> String {
        encode_base32(&self.as_bytes(), AddressNis1::LENGTH_IN_BASE32)
    }
}
