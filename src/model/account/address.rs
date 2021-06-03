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
use std::fmt;
use std::fmt::Debug;
use std::str::FromStr;

use anyhow::{anyhow, ensure, Result};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::{AddressSchema, H256};
use crate::core::format::{decode_base32, public_key_to_address, raw_prettify};
use crate::helpers::H192;
use crate::helpers::is_hex;
use crate::network::NetworkType;

pub type AddressSym = Address<H192>;

/// The `Address` struct describes an Symbol address with its network.
///
#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Hash)]
pub struct Address<H> {
    /// The Symbol address in `H192`.
    pub address: H,

    /// The Symbol network type.
    pub network_type: NetworkType,
}

impl<H: AddressSchema> Address<H> {
    /// Get the `Address` in an raw address string format.
    ///
    /// For example: TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q
    pub fn address_str(&self) -> String {
        self.address.to_base32()
    }

    /// Converts `Address` String into a more readable/pretty format,
    /// a Symbol prettify address string looks like:
    ///
    /// * Before: TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q
    /// * After: TATNE7-Q5BITM-UTRRN6-IB4I7F-LSDRDW-ZA37JG-O5Q
    pub fn prettify(&self) -> String {
        raw_prettify(&self.address_str(), self.address.size_suffix())
    }
}

impl AddressSym {
    /// The length of the Symbol `Address` in base32 string.
    const LENGTH_IN_BASE32: usize = 39;

    /// Creates an Symbol `Address` from a given public_key string.
    ///
    /// # Inputs
    ///
    /// * `public_key`: representing the hex publick key (String or &str).
    ///
    /// * `network_type`: The `NetworkType` of Sybol Blockchain.
    ///
    /// # Example
    ///
    /// ```
    /// use symbol_sdk::account::Address;
    /// use symbol_sdk::network::NetworkType;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let public_key: &str = "2E834140FD66CF87B254A693A2C7862C819217B676D3943267156625E816EC6F";
    /// let address = Address::from_public_key(public_key, NetworkType::TEST_NET).unwrap();
    /// # println!("{}", address);
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an Symbol `Address` or whose error value
    /// is an `Error` describing the error that occurred.
    pub fn from_public_key(public_key: &str, network_type: NetworkType) -> Result<Self> {
        ensure!(is_hex(public_key), "public_key it's not hex.");

        let public_key_hash =
            H256::from_str(public_key).map_err(|e| anyhow!("public_key {}", e))?;

        let address_vec =
            public_key_to_address::<sha3::Sha3_256, H192>(public_key_hash, network_type, 3);

        Ok(Self {
            address: H192::from_slice(address_vec.as_slice()),
            network_type,
        })
    }

    /// Creates an Symbol 'Address' from a given raw address string.
    ///
    /// # Info
    ///
    /// A Symbol raw address string looks like:
    /// * TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q or TATNE7-Q5BITM-UTRRN6-IB4I7F-LSDRDW-ZA37JG-O5Q.
    ///
    /// # Inputs
    ///
    /// * `raw_address`: an `S` representing address (String or &str).
    ///
    /// # Example
    ///
    /// ```
    /// use symbol_sdk::account::Address;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let raw_address: &str = "TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q";
    /// let address = Address::from_raw(raw_address).unwrap();
    /// # println!("{}", address);
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an Symbol `Address` or whose error value
    /// is an `Error` describing the error that occurred.
    pub fn from_raw<S: AsRef<str>>(raw_address: S) -> Result<Self> {
        let address_raw = raw_address.as_ref().trim().replace("-", "");
        ensure!(
            address_raw.len() == Self::LENGTH_IN_BASE32,
            "Invalid raw_address length {} ",
            address_raw.len()
        );

        let network_identifier = address_raw.to_uppercase().chars().next().unwrap();

        let network_type = NetworkType::try_from(network_identifier)?;

        let mut address = H192::zero();
        decode_base32(address.as_mut(), &address_raw);

        Ok(Self {
            address,
            network_type,
        })
    }

    /// Create an Symbol `Address` from the given hex string address
    ///
    /// A hex string address looks like: 908E2C873E8552039933562AFB74A193B48BDD300BEBDB93.
    ///
    /// # Inputs
    ///
    /// * `encoded`: an `S` representing address hex (String or &str).
    ///
    /// # Example
    ///
    /// ```
    /// use symbol_sdk::account::Address;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let encoded: &str = "908E2C873E8552039933562AFB74A193B48BDD300BEBDB93";
    /// let address = Address::from_encoded(encoded).unwrap();
    /// # println!("{}", address);
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an Symbol `Address` or whose error value
    /// is an `Error` describing the error that occurred.
    pub fn from_encoded<S: AsRef<str>>(encoded: S) -> Result<Self> {
        ensure!(is_hex(encoded.as_ref()), "encoded address it's not hex.");

        let address =
            H192::from_str(encoded.as_ref()).map_err(|e| anyhow!("encoded address {}", e))?;
        Ok(Self {
            address,
            network_type: NetworkType::try_from(address.0[0])?,
        })
    }
}

impl<H: Serialize> fmt::Display for Address<H> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

impl<H: Serialize> Serialize for Address<H> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut rgb = serializer.serialize_struct("Address", 2)?;
        rgb.serialize_field("address", &self.address)?;
        rgb.serialize_field("network_type", &self.network_type)?;
        rgb.end()
    }
}
