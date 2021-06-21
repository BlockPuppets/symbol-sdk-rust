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

use crate::core::format::{decode_base32, encode_base32};

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

construct_fixed_hash! {
    /// Symbol 192 bit hash type.
    pub struct H192(24);
}

impl H192 {
    pub fn as_base32(&self) -> String {
        encode_base32(self.as_bytes())
    }

    pub fn from_base32(data: &str) -> Self {
        let mut address = H192::zero();
        decode_base32(address.as_mut(), &data);
        address
    }
}

impl<'de> Deserialize<'de> for H192 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        Ok(H192::from_base32(string.as_ref()))
    }
}

impl Serialize for H192 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.as_base32())
    }
}
