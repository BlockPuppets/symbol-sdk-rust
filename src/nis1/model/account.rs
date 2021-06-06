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
use std::str::FromStr;

use anyhow::{anyhow, ensure, Result};
use crypto::prelude::{KeyPairSchema, PrivateKey, PublicKey};
use hex::ToHex;

use crate::{H200, H256, is_hex, KpNis1};
use crate::account::{Account, Address, PublicAccount, sign_data, verify_signature};
use crate::core::format::{decode_base32, public_key_to_address};
use crate::network::NetworkType;

pub type AddressNis1 = Address<H200>;
pub type PublicAccountNis1 = PublicAccount<H200>;
pub type AccountNis1 = Account<KpNis1, H200>;

impl AccountNis1 {
    pub fn random(network_type: NetworkType) -> Self {
        let key_pair = <KpNis1>::random();
        Self::from_private_key(key_pair.private_key, network_type)
    }

    pub fn from_private_key(private_key: PrivateKey, network_type: NetworkType) -> Self {
        let key_pair = <KpNis1>::from_private_key(private_key);
        Self::from_hex_private_key(
            key_pair.private_key.encode_hex_upper::<String>(),
            network_type,
        )
            .unwrap()
    }

    pub fn from_hex_private_key<S: AsRef<str>>(
        private_key: S,
        network_type: NetworkType,
    ) -> Result<Self> {
        ensure!(is_hex(private_key.as_ref()), "private_key it's not hex.");

        let key_pair = <KpNis1>::from_hex_private_key(private_key.as_ref())?;

        let public_key = key_pair.public_key().encode_hex::<String>();
        let public_account = PublicAccountNis1::from_public_key(public_key, network_type)?;

        Ok(Self {
            key_pair,
            public_account,
        })
    }

    pub fn sign_data(&self, data: &str) -> Result<crypto::prelude::Signature> {
        sign_data::<KpNis1>(self.key_pair, data)
    }

    pub fn verify_signature(
        &self,
        data: &str,
        signature: crypto::prelude::Signature,
    ) -> Result<()> {
        self.public_account
            .verify_signature(data.as_ref(), signature)
    }
}

impl PublicAccountNis1 {
    pub fn from_public_key<S: AsRef<str>>(
        public_key: S,
        network_type: NetworkType,
    ) -> Result<Self> {
        let address = AddressNis1::from_public_key(public_key.as_ref(), network_type)?;
        Ok(Self {
            address,
            public_key: PublicKey::from_str(public_key.as_ref()).unwrap(),
        })
    }

    pub fn verify_signature(
        &self,
        data: &str,
        signature: crypto::prelude::Signature,
    ) -> Result<()> {
        verify_signature::<KpNis1>(self.public_key, data, signature)
    }
}

impl AddressNis1 {
    /// The length of the Nis1 `Address` in base32 string.
    const LENGTH_IN_BASE32: usize = 40;

    pub fn from_public_key(public_key: &str, network_type: NetworkType) -> Result<Self> {
        ensure!(is_hex(public_key), "public_key it's not hex.");

        Self::__internal_valid_network_type(network_type)?;

        let public_key_hash =
            H256::from_str(public_key).map_err(|e| anyhow!("public_key {}", e))?;

        let address_vec =
            public_key_to_address::<sha3::Keccak256, H200>(public_key_hash, network_type, 4);

        Ok(Self {
            address: H200::from_slice(address_vec.as_slice()),
            network_type,
        })
    }

    pub fn from_raw<S: AsRef<str>>(raw_address: S) -> Result<Self> {
        let address_raw = raw_address.as_ref().trim().replace("-", "");
        ensure!(
            address_raw.len() == Self::LENGTH_IN_BASE32,
            "Invalid raw_address length {} ",
            address_raw.len()
        );

        let network_identifier = address_raw.to_uppercase().chars().next().unwrap();

        let network_type = NetworkType::try_from(network_identifier)?;

        Self::__internal_valid_network_type(network_type)?;

        let mut address = H200::zero();
        decode_base32(address.as_mut(), &address_raw);

        Ok(Self {
            address,
            network_type,
        })
    }

    // internal function
    fn __internal_valid_network_type(network_type: NetworkType) -> Result<()> {
        ensure!(
            network_type != NetworkType::MIJIN_TEST
                && network_type != NetworkType::PRIVATE
                && network_type != NetworkType::PRIVATE_TEST,
            format!("Invalid NetworkType \"{}\"", network_type)
        );
        Ok(())
    }
}
