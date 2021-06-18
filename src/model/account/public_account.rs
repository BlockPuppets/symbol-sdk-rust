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

use anyhow::Result;
use crypto::prelude::{KeyPairSchema, PublicKey};
use hex::ToHex;
use serde::Serialize;

use crate::account::Address;
use crate::network::NetworkType;
use crate::{der_from_hex_upper, is_hex, ser_to_hex_upper};

/// The `PublicAccount` struct contains account's Symbol `Address` and public key.
///
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct PublicAccount {
    /// the Symbol account's `Address`.
    pub address: Address,
    /// the Symbol account public key in `crypto PublicKey`.
    #[serde(
        serialize_with = "ser_to_hex_upper",
        deserialize_with = "der_from_hex_upper"
    )]
    pub public_key: PublicKey,
}

impl PublicAccount {
    /// Account public key to hex String.
    ///
    pub fn public_key_to_hex(&self) -> String {
        self.public_key.encode_hex::<String>()
    }

    /// Account `NetworkType`.
    ///
    pub fn network_type(&self) -> NetworkType {
        self.address.network_type
    }

    /// Get the `Address` in an raw address string format.
    ///
    /// Example: TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q
    pub fn address_str(&self) -> String {
        self.address.address_str()
    }

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
    /// let public_key: &str = "2E834140FD66CF87B254A693A2C7862C819217B676D3943267156625E816EC6F";
    /// let public_account = PublicAccount::from_public_key(public_key,
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
        let address = Address::from_public_key(public_key.as_ref(), network_type)?;
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
    pub fn verify_signature(
        &self,
        data: &str,
        signature: crypto::prelude::Signature,
    ) -> Result<()> {
        verify_signature(self.public_key, data, signature)
    }
}

impl fmt::Display for PublicAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

// internal function.
pub(crate) fn verify_signature(
    public_key: PublicKey,
    data: &str,
    signature: crypto::prelude::Signature,
) -> Result<()> {
    // ensure!(!data.is_empty(), "data cannot be empty");

    let kp = crypto::sym::Keypair::from_null_private_key(public_key);

    let signature: crypto::prelude::Signature = (signature.as_fixed_bytes()).into();

    let data = if is_hex(data) {
        hex::decode(data)?
    } else {
        Vec::from(data)
    };

    kp.verify(data.as_ref(), signature)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crypto::prelude::Signature;

    use crate::account::{Account, PublicAccount};
    use crate::network::NetworkType;

    const PUBLIC_KEY: &str = "b4f12e7c9f6946091e2cb8b6d3a12b50d17ccbbf646386ea27ce2946a7423dcf";

    #[test]
    fn test_should_create_from_public_key() {
        let public_account =
            PublicAccount::from_public_key(PUBLIC_KEY, NetworkType::PRIVATE_TEST).unwrap();
        assert_eq!(public_account.public_key_to_hex(), PUBLIC_KEY);
        assert_eq!(
            public_account.address_str(),
            "VARNASAS2BIAB6LMFA3FPMGBPGIJGK6IJGOH3FA"
        );
    }

    #[test]
    fn test_can_verify_a_signature() {
        let testing_account: Account = crate::account::account::tests::TESTING_ACCOUNT.clone();

        let signer_public_account = testing_account.public_account;
        let data =
            "ff60983e0c5d21d2fb83c67598d560f3cf0e28ae667b5616aaa58a059666cd8cf826b026243c92cf";
        let signature = testing_account.sign_data(data).unwrap();
        assert!(signer_public_account
            .verify_signature(data, signature)
            .is_ok());
    }

    #[test]
    #[should_panic(expected = "Invalid input length")]
    fn test_return_panic_if_signature_hash_invalid_length() {
        let signer_public_account = PublicAccount::from_public_key(
            "22816F825B4CACEA334723D51297D8582332D8B875A5829908AAE85831ABB508",
            NetworkType::PRIVATE_TEST,
        )
        .unwrap();

        let data = "I am so so so awesome as always";
        let signature = Signature::from_str("B01DCA6484026C2ECDF3C822E64DEAAFC15EBCCE337EEE209C28513CB5351CDED8863A8E7B855CD471B55C91FAE611C5486").unwrap();
        let _ = signer_public_account.verify_signature(data, signature);
    }

    #[test]
    #[should_panic(expected = "Invalid character 'w' at position 123")]
    fn test_return_panic_if_is_not_strictly_hexadecimal() {
        let signer_public_account = PublicAccount::from_public_key(
            "22816F825B4CACEA334723D51297D8582332D8B875A5829908AAE85831ABB508",
            NetworkType::PRIVATE_TEST,
        )
        .unwrap();

        let data = "I am so so so awesome as always";
        let signature = Signature::from_str("B01DCA6484026C2ECDF3C822E64DEAAFC15EBCCE337EEE209C28513CB5351CDED8863A8E7B855CD471B55C91FAE611C548625C9A5916A555A24F72F35a1wwwww").unwrap();
        let _ = signer_public_account.verify_signature(data, signature);
    }

    #[test]
    fn test_return_false_if_wrong_public_key_provided() {
        let signer_public_account = PublicAccount::from_public_key(
            "12816F825B4CACEA334723D51297D8582332D8B875A5829908AAE85831ABB509",
            NetworkType::PRIVATE_TEST,
        )
        .unwrap();

        let data = "I am so so so awesome as always";
        let signature = Signature::from_str("B01DCA6484026C2ECDF3C822E64DEAAFC15EBCCE337EEE209C28513CB5351CDED8863A8E7B855CD471B55C91FAE611C548625C9A5916A555A24F72F3526FA508").unwrap();
        assert!(signer_public_account
            .verify_signature(data, signature)
            .is_err());
    }

    #[test]
    fn test_return_false_if_data_is_not_corresponding_to_signature_provided() {
        let signer_public_account = PublicAccount::from_public_key(
            "22816F825B4CACEA334723D51297D8582332D8B875A5829908AAE85831ABB508",
            NetworkType::PRIVATE_TEST,
        )
        .unwrap();

        let data = "I am awesome as always";
        let signature = Signature::from_str("B01DCA6484026C2ECDF3C822E64DEAAFC15EBCCE337EEE209C28513CB5351CDED8863A8E7B855CD471B55C91FAE611C548625C9A5916A555A24F72F3526FA508").unwrap();
        assert!(signer_public_account
            .verify_signature(data, signature)
            .is_err());
    }

    #[test]
    fn test_return_false_if_signature_is_not_corresponding_to_data_provided() {
        let signer_public_account = PublicAccount::from_public_key(
            "22816F825B4CACEA334723D51297D8582332D8B875A5829908AAE85831ABB508",
            NetworkType::PRIVATE_TEST,
        )
        .unwrap();

        let data = "I am so so so awesome as always";
        let signature = Signature::from_str("A01DCA6484026C2ECDF3C822E64DEAAFC15EBCCE337EEE209C28513CB5351CDED8863A8E7B855CD471B55C91FAE611C548625C9A5916A555A24F72F3526FA509").unwrap();
        assert!(signer_public_account
            .verify_signature(data, signature)
            .is_err());
    }
}
