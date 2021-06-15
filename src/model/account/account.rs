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
use crypto::prelude::{KeyPairSchema, PrivateKey, Signature};
use hex::ToHex;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

use crate::account::PublicAccount;
use crate::message::{EncryptedMessage, PlainMessage};
use crate::network::NetworkType;
use crate::{is_hex, AddressSchema, GenerationHash, KpSym, H192};

pub type AccountSym = Account<KpSym, H192>;

/// The `Account` struct contains account's `Keypair` and `PublicAccount`.
///
#[derive(Debug, Clone, Deserialize, PartialEq, Hash)]
pub struct Account<Kp: KeyPairSchema, H: AddressSchema> {
    /// The keyPair containing the public and private key of this account.
    pub key_pair: Kp,
    /// The public account of this account.
    pub public_account: PublicAccount<H>,
}

impl<Kp: KeyPairSchema, H: AddressSchema> Account<Kp, H> {
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
        self.key_pair.private_key().encode_hex_upper::<String>()
    }

    /// Sign raw data.
    ///
    pub fn sign_data(&self, data: &str) -> Result<crypto::prelude::Signature> {
        sign_data::<Kp>(self.key_pair, data)
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
        recipient_public_account: PublicAccount<H>,
    ) -> Result<EncryptedMessage> {
        let message = message.as_ref();

        ensure!(!message.is_empty(), "message must not be empty.");

        EncryptedMessage::create::<Kp::Crypto>(
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
        signer_public_account: PublicAccount<H>,
    ) -> Result<PlainMessage> {
        EncryptedMessage::decrypt::<Kp::Crypto>(
            encrypted_message,
            &self.private_key_to_hex(),
            &signer_public_account.public_key_to_hex(),
        )
    }
}

impl AccountSym {
    /// Creates an Symbol `Account` random.
    ///
    /// # Inputs
    ///
    /// * `network_type`: The `NetworkType` of Sybol Blockchain.
    ///
    /// # Example
    ///
    /// ```
    /// use symbol_sdk::Sym;
    /// use symbol_sdk::account::Account;
    /// use symbol_sdk::network::NetworkType;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let account = Account::<KpSym>::random(NetworkType::TEST_NET);
    /// # println!("{}", account);
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// A Symbol `Account`.
    pub fn random(network_type: NetworkType) -> Self {
        let key_pair = <KpSym>::random();
        let public_key = key_pair.public_key().encode_hex::<String>();
        let public_account =
            PublicAccount::<H192>::from_public_key(public_key, network_type).unwrap();

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
    /// use symbol_sdk::Sym;
    /// use symbol_sdk::account::Account;
    /// use symbol_sdk::network::NetworkType;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let private_key: &str = "75027D85CE92E2C469297F4C91E4E88AE03868A91B23C835AEF7C5EFDAD0DBDB";
    /// let account = Account::<KpSym>::from_hex_private_key(private_key, NetworkType::TEST_NET).unwrap();
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

        let key_pair = <KpSym>::from_hex_private_key(private_key.as_ref())?;

        let public_key = key_pair.public_key().encode_hex::<String>();
        let public_account = PublicAccount::<H192>::from_public_key(public_key, network_type)?;

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
    /// use symbol_sdk::Sym;
    /// use symbol_sdk::account::Account;
    /// use symbol_sdk::network::NetworkType;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let (account, mnemonic) = Account::<KpSym>::create_with_mnemonic("any_password", NetworkType::TEST_NET).unwrap();
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
    /// use symbol_sdk::Sym;
    /// use symbol_sdk::account::Account;
    /// use symbol_sdk::network::NetworkType;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let mnemonic: &str = r"force night tumble pole record inflict idea bone deal section
    ///                         essay razor hunt kiwi drill include rifle broken lucky infant
    ///                         satoshi sweet boss blue";
    /// let account = Account::<KpSym>::from_mnemonic(mnemonic , "any_password", NetworkType::TEST_NET).unwrap();
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
        _cosignatories: Vec<Account<KpSym, H192>>,
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

impl<Kp: KeyPairSchema, H: AddressSchema + Serialize> fmt::Display for Account<Kp, H> {
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

impl<Kp: KeyPairSchema + serde::Serialize, H: AddressSchema + Serialize> Serialize
    for Account<Kp, H>
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut rgb = serializer.serialize_struct("Account", 2)?;
        rgb.serialize_field("keypair", &self.key_pair)?;
        rgb.serialize_field("public_account", &self.public_account)?;
        rgb.end()
    }
}

// internal function.
pub(crate) fn sign_data<Kp: KeyPairSchema>(kp: Kp, data: &str) -> Result<Signature> {
    ensure!(!data.is_empty(), "data cannot be empty");

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
    use crate::account::{Account, AccountSym};
    use crate::network::NetworkType;
    use crate::{KpSym, H192};

    lazy_static! {
        pub static ref TESTING_ACCOUNT: Account<KpSym, H192> = AccountSym::from_hex_private_key(
            "26b64cb10f005e5988a36744ca19e20d835ccc7c105aaa5f3b212da593180930",
            NetworkType::PRIVATE_TEST
        )
        .unwrap();
    }
}
