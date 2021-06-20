use std::str::FromStr;

use crypto::prelude::PublicKey;
use hex::ToHex;

pub fn ser_to_hex_upper<D: AsRef<[u8]>, S>(data: &D, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
{
    serializer.serialize_str(&data.encode_hex_upper::<String>())
}

pub fn der_from_hex_upper<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
    where
        D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    PublicKey::from_str(&s).map_err(serde::de::Error::custom)
}
