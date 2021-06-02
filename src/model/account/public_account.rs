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
use std::fmt::Debug;
use std::str::FromStr;

use anyhow::{ensure, Result};
use crypto::{KeyPairSchema, PublicKey};
use hex::ToHex;
use serde::Serialize;

use crate::{AddressSchema, H192, is_hex, Sym};
use crate::account::{Address, AddressSym};
use crate::network::NetworkType;

pub type PublicAccountSym = PublicAccount<H192>;

/// The `PublicAccount` struct contains account's Symbol `Address` and public key.
///
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct PublicAccount<H> {
    /// the Symbol account's `Address`.
    pub address: Address<H>,
    /// the Symbol account public key in `crypto PublicKey`.
    pub public_key: PublicKey,
}

impl<H: AddressSchema> PublicAccount<H> {
    /// Account public key to hex String.
    ///
    pub fn public_key_to_hex(&self) -> String {
        self.public_key.encode_hex_upper::<String>()
    }

    /// Account `NetworkType`.
    ///
    pub fn network_type(&self) -> NetworkType {
        self.address.network_type
    }

    /// Get the `Address` in an raw address string format.
    ///
    /// For Symbol example: TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q
    /// For Nis1 example:   TBL72JIXHCEBT37B3WBAE5LLP3H6SZM6QFDC2GO7
    pub fn address_str(&self) -> String {
        self.address.address_str()
    }
}

impl PublicAccountSym {
    /// Creates an Symbol `PublicAccount` from a given public_key string.
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
    /// use symbol_sdk::account::PublicAccount;
    /// use symbol_sdk::network::NetworkType;
    ///
    /// #
    /// # fn main() {
    /// #
    /// use symbol_sdk::Sym;
    /// let public_key: &str = "2E834140FD66CF87B254A693A2C7862C819217B676D3943267156625E816EC6F";
    /// let public_account = PublicAccount::<Sym>::from_public_key(public_key,
    /// NetworkType::TEST_NET)
    /// .unwrap();
    /// # println!("{}", public_account);
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an Symbol `PublicAccount` or whose error value
    /// is an `Error` describing the error that occurred.
    pub fn from_public_key<S: AsRef<str>>(
        public_key: S,
        network_type: NetworkType,
    ) -> Result<Self> {
        let address = AddressSym::from_public_key(public_key.as_ref(), network_type)?;
        Ok(Self {
            address,
            public_key: PublicKey::from_str(public_key.as_ref()).unwrap(),
        })
    }

    /// Verify a signature.
    ///
    /// # Inputs
    ///
    /// * `data`: representing the data to verify.
    ///
    /// * `signature`: a `Signature` representing the signature to verify.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value True if the signature is valid or whose error value
    /// is an `Error` describing the error that occurred.
    pub fn verify_signature(&self, data: &str, signature: crypto::Signature) -> Result<()> {
        verify_signature::<Sym>(self.public_key, data, signature)
    }
}

impl<H: AddressSchema + Serialize> fmt::Display for PublicAccount<H> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

// internal function.
pub(crate) fn verify_signature<Kp: KeyPairSchema>(
    public_key: PublicKey,
    data: &str,
    signature: crypto::Signature,
) -> Result<()> {
    ensure!(!data.is_empty(), "data cannot be empty");

    let kp = <Kp>::from_null_private_key(public_key);

    let signature: crypto::Signature = (signature.as_fixed_bytes()).into();

    let data = if is_hex(data) {
        hex::decode(data)?
    } else {
        Vec::from(data)
    };

    kp.verify(data.as_ref(), signature)
}
