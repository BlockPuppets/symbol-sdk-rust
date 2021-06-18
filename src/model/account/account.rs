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

use anyhow::{ensure, Result};
use crypto::{
    prelude::{KeyPairSchema, PrivateKey, Signature},
    sym::Keypair,
};
use hex::ToHex;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::{GenerationHash, is_hex};
use crate::account::PublicAccount;
use crate::message::{EncryptedMessage, PlainMessage};
use crate::network::NetworkType;

/// The `Account` struct contains account's `Keypair` and `PublicAccount`.
///
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Account {
    /// The keyPair containing the public and private key of this account.
    pub key_pair: Keypair,
    /// The public account of this account.
    pub public_account: PublicAccount,
}

impl Account {
    /// Get the `Address` in an raw address string format.
    ///
    /// For example: TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q
    pub fn address_str(&self) -> String {
        self.public_account.address_str()
    }

    ///  account `NetworkType`.
    ///
    pub fn network_type(&self) -> NetworkType {
        self.public_account.network_type()
    }

    ///  Account public key to hex String.
    ///
    pub fn public_key_to_hex(&self) -> String {
        self.public_account.public_key_to_hex()
    }

    ///  Account private key to hex String.
    ///
    pub fn private_key_to_hex(&self) -> String {
        self.key_pair.private_key().encode_hex::<String>()
    }

    /// Sign raw data.
    ///
    pub fn sign_data(&self, data: &str) -> Result<crypto::prelude::Signature> {
        sign_data(self.key_pair, data)
    }

    /// Creates an encrypted message from this account to the recipient PublicAccount.
    ///
    /// # Inputs
    ///
    /// * `message`: Plain message to be encrypted.
    /// * `recipient_public_account`: Recipient public account.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an `EncryptedMessage` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub fn encrypt_message<S: AsRef<str>>(
        &self,
        message: S,
        recipient_public_account: PublicAccount,
    ) -> Result<EncryptedMessage> {
        let message = message.as_ref();

        ensure!(!message.is_empty(), "message must not be empty.");

        EncryptedMessage::create(
            &message.as_bytes(),
            &self.private_key_to_hex(),
            &recipient_public_account.public_key_to_hex(),
        )
    }

    /// Decrypts an `EncryptedMessage` received by this account from senderPublicAccount.
    ///
    /// # Inputs
    ///
    /// * `encrypted_message`: Encrypted message.
    /// * `signer_public_account`: The public account originally encrypted the message.
    ///
    /// A `Result` whose okay value is an `PlainMessage` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub fn decrypt_message(
        &self,
        encrypted_message: &EncryptedMessage,
        signer_public_account: PublicAccount,
    ) -> Result<PlainMessage> {
        EncryptedMessage::decrypt(
            encrypted_message,
            &self.private_key_to_hex(),
            &signer_public_account.public_key_to_hex(),
        )
    }
    /// Creates an Symbol `Account` random.
    ///
    /// # Inputs
    ///
    /// * `network_type`: The `NetworkType` of Sybol Blockchain.
    ///
    /// # Example
    ///
    /// ```
    /// use symbol_sdk::account::Account;
    /// use symbol_sdk::network::NetworkType;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let account = Account::random(NetworkType::TEST_NET);
    /// # println!("{}", account);
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// A Symbol `Account`.
    pub fn random(network_type: NetworkType) -> Self {
        let key_pair = Keypair::random();
        let public_key = key_pair.public_key().encode_hex::<String>();
        let public_account = PublicAccount::from_public_key(public_key, network_type).unwrap();

        Self {
            key_pair,
            public_account,
        }
    }

    /// Creates an Symbol `Account` from a given hex private key string.
    ///
    /// # Inputs
    ///
    /// * `private_key`: representing the hex private key (String or &str).
    ///
    /// * `network_type`: The `NetworkType` of Symbol Blockchain.
    ///
    /// # Example
    ///
    /// ```
    /// use symbol_sdk::account::Account;
    /// use symbol_sdk::network::NetworkType;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let private_key: &str = "75027D85CE92E2C469297F4C91E4E88AE03868A91B23C835AEF7C5EFDAD0DBDB";
    /// let account = Account::from_hex_private_key(private_key, NetworkType::TEST_NET).unwrap();
    /// # println!("{}", account);
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an Symbol `Account` or whose error value
    /// is an `Error` describing the error that occurred.
    pub fn from_hex_private_key<S: AsRef<str>>(
        private_key: S,
        network_type: NetworkType,
    ) -> Result<Self> {
        ensure!(is_hex(private_key.as_ref()), "private_key it's not hex.");

        let key_pair = Keypair::from_hex_private_key(private_key.as_ref())?;

        let public_key = key_pair.public_key().encode_hex::<String>();
        let public_account = PublicAccount::from_public_key(public_key, network_type)?;

        Ok(Self {
            key_pair,
            public_account,
        })
    }

    /// Create a new Symbol `Account` with a BIP-39 Mnemonic using a cryptographically secure random
    /// number generator.
    ///
    /// # Inputs
    ///
    /// * `password`: representing password key (String or &str).
    ///
    /// * `network_type`: The `NetworkType` of Symbol Blockchain.
    ///
    /// # Example
    ///
    /// ```
    /// use symbol_sdk::account::Account;
    /// use symbol_sdk::network::NetworkType;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let (account, mnemonic) = Account::create_with_mnemonic("any_password", NetworkType::TEST_NET).unwrap();
    /// # println!("{}", account);
    /// # println!("{}", mnemonic);
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// A Symbol `Account`
    pub fn create_with_mnemonic(
        password: &str,
        network_type: NetworkType,
    ) -> Result<(Self, String)> {
        let (private_key, mnemonic) = PrivateKey::create_with_mnemonic(password)?;
        let account = Self::from_hex_private_key(private_key.encode_hex::<String>(), network_type)?;

        Ok((account, mnemonic))
    }

    /// Re-construct a Symbol `Account` from the supplied mnemonic and password.
    ///
    /// # Inputs
    ///
    /// * `mnemonic`: representing the mnemonic (String or &str).
    ///
    /// * `password`: representing the password key (String or &str).
    ///
    /// * `network_type`: The `NetworkType` of Symbol Blockchain.
    ///
    /// # Example
    ///
    /// ```
    /// use symbol_sdk::account::Account;
    /// use symbol_sdk::network::NetworkType;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let mnemonic: &str = r"force night tumble pole record inflict idea bone deal section
    ///                         essay razor hunt kiwi drill include rifle broken lucky infant
    ///                         satoshi sweet boss blue";
    /// let account = Account::from_mnemonic(mnemonic , "any_password", NetworkType::TEST_NET).unwrap();
    /// # println!("{}", account);
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an Symbol `Account` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub fn from_mnemonic(
        mnemonic: &str,
        password: &str,
        network_type: NetworkType,
    ) -> Result<Self> {
        let secret_key = PrivateKey::from_mnemonic(mnemonic, password)?;

        Self::from_hex_private_key(secret_key.encode_hex::<String>(), network_type)
    }

    /// Verify a signature.
    ///
    pub fn verify_signature(
        &self,
        data: &str,
        signature: crypto::prelude::Signature,
    ) -> Result<()> {
        self.public_account
            .verify_signature(data.as_ref(), signature)
    }

    pub fn sign_transaction(
        &self,
        _transaction: Vec<u8>,
        _generation_hash: GenerationHash,
    ) -> Result<Vec<u8>> {
        unimplemented!()
    }

    pub fn sign_transaction_with_cosignatories(
        &self,
        _transaction: Vec<u8>,
        _cosignatories: Vec<Account>,
        _generation_hash: GenerationHash,
    ) -> Result<Vec<u8>> {
        unimplemented!()
    }

    pub fn sign_transaction_given_signatures(
        &self,
        _transaction: Vec<u8>,
        _cosignature_signed_transactions: Vec<u8>,
        _generation_hash: GenerationHash,
    ) -> Result<Vec<u8>> {
        unimplemented!()
    }

    pub fn sign_cosignature_transaction(
        &self,
        _cosignature_transaction: Vec<u8>,
        _generation_hash: GenerationHash,
    ) -> Result<Vec<u8>> {
        unimplemented!()
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Warn!
        // For security the keypair is not shown with Display.
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self.public_account).unwrap_or_default()
        )
    }
}

// impl Serialize for Account {
//     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut rgb = serializer.serialize_struct("Account", 2)?;
//         rgb.serialize_field("keypair", &self.key_pair)?;
//         rgb.serialize_field("public_account", &self.public_account)?;
//         rgb.end()
//     }
// }

// internal function.
pub(crate) fn sign_data(kp: Keypair, data: &str) -> Result<Signature> {
    // ensure!(!data.is_empty(), "data cannot be empty");

    let data = if is_hex(data) {
        hex::decode(data)?
    } else {
        Vec::from(data)
    };

    let signature = kp.sign(data.as_ref());
    Ok(Signature::from(signature.to_fixed_bytes()))
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::account::Account;
    use crate::network::NetworkType;

    lazy_static! {
        pub static ref TESTING_ACCOUNT: Account = Account::from_hex_private_key(
            "26b64cb10f005e5988a36744ca19e20d835ccc7c105aaa5f3b212da593180930",
            NetworkType::PRIVATE_TEST
        )
        .unwrap();
        pub static ref MULTISIG_ACCOUNT: Account = Account::from_hex_private_key(
            "5edebfdbeb32e9146d05ffd232c8af2cf9f396caf9954289daa0362d097fff3b",
            NetworkType::PRIVATE_TEST
        )
        .unwrap();
        pub static ref COSIGNATORY_ACCOUNT: Account = Account::from_hex_private_key(
            "2a2b1f5d366a5dd5dc56c3c757cf4fe6c66e2787087692cf329d7a49a594658b",
            NetworkType::PRIVATE_TEST
        )
        .unwrap();
        pub static ref COSIGNATORY2_ACCOUNT: Account = Account::from_hex_private_key(
            "b8afae6f4ad13a1b8aad047b488e0738a437c7389d4ff30c359ac068910c1d59",
            NetworkType::PRIVATE_TEST
        )
        .unwrap();
        pub static ref COSIGNATORY3_ACCOUNT: Account = Account::from_hex_private_key(
            "111602be4d36f92dd60ca6a3c68478988578f26f6a02f8c72089839515ab603e",
            NetworkType::PRIVATE_TEST
        )
        .unwrap();
    }

    #[cfg(test)]
    mod tests_account {
        use super::*;

        const ADDRESS: &str = "VDLGYM2CBZKBDGK3VT6KFMUM6HE7LXL2WGA37KA";
        const PUBLIC_KEY: &str = "9801508C58666C746F471538E43002B85B1CD542F9874B2861183919BA8787B6";
        const PRIVATE_KEY: &str =
            "26b64cb10f005e5988a36744ca19e20d835ccc7c105aaa5f3b212da593180930";

        #[test]
        fn test_should_create_from_private_key() {
            let account =
                Account::from_hex_private_key(PRIVATE_KEY, NetworkType::PRIVATE_TEST).unwrap();
            assert_eq!(account.public_key_to_hex(), PUBLIC_KEY.to_lowercase());
            assert_eq!(account.private_key_to_hex(), PRIVATE_KEY.to_lowercase());

            assert_eq!(account.address_str(), ADDRESS);
        }

        #[test]
        fn test_should_return_error_when_the_private_key_is_not_valid() {
            let account = Account::from_hex_private_key("", NetworkType::PRIVATE_TEST);
            assert!(account.is_err());
        }

        #[test]
        fn test_should_generate_a_new_account() {
            let account = Account::random(NetworkType::PRIVATE_TEST);

            assert_ne!(account.private_key_to_hex(), "");
            assert_ne!(account.public_key_to_hex(), "");
            assert_ne!(account.address_str(), "");
        }

        #[test]
        fn test_should_return_network_type() {
            let account = Account::random(NetworkType::TEST_NET);
            assert_eq!(account.network_type(), NetworkType::TEST_NET);
        }
    }

    #[cfg(test)]
    mod tests_sign_data {
        use super::*;

        const PRIVATE_KEY: &str =
            "AB860ED1FE7C91C02F79C02225DAC708D7BD13369877C1F59E678CC587658C47";

        #[test]
        fn test_utf8_data() {
            let account =
                Account::from_hex_private_key(PRIVATE_KEY, NetworkType::PRIVATE_TEST).unwrap();

            let data = "catapult rocks!";

            let public_account = account.public_account;
            let signed = account.sign_data(data).unwrap();
            assert!(public_account.verify_signature(data, signed).is_ok());
        }

        #[test]
        fn test_hex_data() {
            let account =
                Account::from_hex_private_key(PRIVATE_KEY, NetworkType::PRIVATE_TEST).unwrap();

            let data = "0xAA";

            let public_account = account.public_account;
            let signed = account.sign_data(data).unwrap();
            assert!(public_account.verify_signature(data, signed).is_ok());
        }

        #[test]
        fn test_hexa_without_0x_prefix_should_be_the_same_as_with_0x() {
            let account =
                Account::from_hex_private_key(PRIVATE_KEY, NetworkType::PRIVATE_TEST).unwrap();

            let public_account = account.public_account;

            let signed = account.sign_data("AA").unwrap();
            let signed_with0x = account.sign_data("0xAA").unwrap();

            assert!(public_account.verify_signature("AA", signed).is_ok());
            assert!(public_account
                .verify_signature("0xAA", signed_with0x)
                .is_ok());
        }

        #[test]
        fn test_sign_empty() {
            let account =
                Account::from_hex_private_key(PRIVATE_KEY, NetworkType::PRIVATE_TEST).unwrap();

            let public_account = account.public_account;

            let signed = account.sign_data("").unwrap();
            let signed_with0x = account.sign_data("0x").unwrap();

            assert!(public_account.verify_signature("", signed).is_ok());
            assert!(public_account.verify_signature("0x", signed_with0x).is_ok());
        }
    }
}
