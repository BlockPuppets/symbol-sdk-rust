/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::convert::TryFrom;
use std::fmt;

use anyhow::Result;
use crypto::prelude::KeyPairSchema;
use sha3::{Digest, Sha3_256};

use crate::{GenerationHash, H256, hex_decode};
use crate::account::Account;
use crate::model::transaction::{SignedTransaction, TransactionType};
use crate::network::NetworkType;
use crate::transaction::common_transaction::CommonTransaction;

/// An abstract transaction trait that serves as the base of all transaction types.
///
#[typetag::serde]
pub trait Transaction: Sync + Send
    where
        Self: fmt::Debug,
{
    fn serializer(&self) -> Vec<u8>;

    /// Get the `CommonTransaction`.
    fn get_common_transaction(&self) -> &CommonTransaction;

    /// Get the `NetworkType`.
    fn get_network_type(&self) -> NetworkType {
        self.get_common_transaction().network_type
    }

    /// Get the `TransactionType`.
    fn get_transaction_type(&self) -> TransactionType {
        self.get_common_transaction().transaction_type
    }

    /// Get the transaction hash.
    fn get_transaction_hash(&self) -> H256 {
        self.get_common_transaction().get_transaction_hash()
    }

    /// Get the `EmbeddedTransactionHelper` buffer.
    fn to_embedded_transaction_builder(
        &self,
    ) -> Box<dyn buffer::embedded_transaction_helper::EmbeddedTransactionHelper>;

    /// Generate signing bytes.
    ///
    /// # Inputs
    ///
    /// * `payload_bytes`: Payload buffer.
    /// * `generation_hash_bytes`: GenerationHash buffer.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>`.
    fn get_signing_bytes(&self, payload_bytes: &[u8], generation_hash_bytes: &[u8]) -> Vec<u8> {
        let byte_buffer_without_header = &payload_bytes[4 + 64 + 32 + 8..];
        if self.get_transaction_type() == TransactionType::AggregateBonded
            || self.get_transaction_type() == TransactionType::AggregateComplete
        {
            [
                generation_hash_bytes[..52].as_ref(),
                byte_buffer_without_header,
            ]
                .concat()
        } else {
            [generation_hash_bytes, byte_buffer_without_header].concat()
        }
    }

    /// Serialize and sign transaction creating a SignedTransaction.
    ///
    /// # Inputs
    ///
    /// * `account`: The account to sign the transaction.
    /// * `generation_hash`: Network generation hash hex.
    ///
    /// # Returns
    ///
    /// A Symbol `SignedTransaction`.
    fn sign_with(
        &self,
        account: Account,
        generation_hash: GenerationHash,
    ) -> Result<SignedTransaction> {
        let generation_hash_bytes = generation_hash.to_fixed_bytes();
        let transaction_buffer = self.serializer();

        let signing_bytes =
            self.get_signing_bytes(transaction_buffer.as_ref(), generation_hash_bytes.as_ref());
        let signature = account.key_pair.sign(signing_bytes.as_ref());
        let mut signed_transaction_buffer = Vec::with_capacity(transaction_buffer.len());
        signed_transaction_buffer.extend_from_slice(signature.as_bytes());
        signed_transaction_buffer.extend_from_slice(account.public_account.public_key.as_ref());
        signed_transaction_buffer.extend_from_slice([0u8; 4].as_ref());
        signed_transaction_buffer
            .extend_from_slice(transaction_buffer[64 + 32 + 4..transaction_buffer.len()].as_ref());
        let payload = hex::encode(signed_transaction_buffer);

        Ok(SignedTransaction {
            payload: payload.to_uppercase(),
            hash: Transaction::create_transaction_hash(&payload, &generation_hash_bytes)?,
            signer_public_key: account.public_account,
            _type: self.get_transaction_type(),
            network_type: self.get_network_type(),
        })
    }

    /// Transaction pending to be included in a block.
    fn is_unconfirmed(&self) -> bool {
        let common = self.get_common_transaction().clone();
        if let Some(transaction_info) = common.transaction_info {
            transaction_info.height == 0
                && transaction_info.hash.is_some()
                && transaction_info.merkle_component_hash.is_some()
                && transaction_info.hash.unwrap_or_default()
                == transaction_info.merkle_component_hash.unwrap_or_default()
        } else {
            false
        }
    }

    /// Transaction included in a block.
    fn is_confirmed(&self) -> bool {
        let common = self.get_common_transaction().clone();
        if let Some(transaction_info) = common.transaction_info {
            transaction_info.height > 0
        } else {
            false
        }
    }

    /// if a transaction has missing signatures.
    fn has_missing_signatures(&self) -> bool {
        let common = self.get_common_transaction().clone();
        if let Some(transaction_info) = common.transaction_info {
            transaction_info.height == 0
                && transaction_info.hash.is_some()
                && transaction_info.merkle_component_hash.is_some()
                && transaction_info.hash.unwrap_or_default()
                != transaction_info.merkle_component_hash.unwrap_or_default()
        } else {
            false
        }
    }

    /// Transaction is not known by the network
    fn is_unannounced(&self) -> bool {
        self.get_common_transaction().transaction_info.is_none()
    }
}

impl dyn Transaction {
    /// Transaction header size
    ///
    /// Included fields are `size`, `verifiableEntityHeader_Reserved1`, `signature`, `signerPublicKey` and `entityBody_Reserved1`.
    ///
    const HEADER_SIZE: usize = 8 + 64 + 32 + 4;

    /// Index of the transaction *type*
    ///
    /// Included fields are the transaction header, `version` and `network`
    const TYPE_INDEX: usize = Self::HEADER_SIZE + 2;

    /// Index of the transaction *body*
    ///
    /// Included fields are the transaction header, `version`, `network`, `type`, `maxFee` and `deadline`
    const BODY_INDEX: usize = Self::HEADER_SIZE + 1 + 1 + 2 + 8 + 8;

    /// Generate transaction hash hex.
    ///
    /// # See
    /// https://github.com/nemtech/catapult-server/blob/main/src/catapult/model/EntityHasher.cpp#L32
    ///
    /// # See
    /// https://github.com/nemtech/catapult-server/blob/main/src/catapult/model/EntityHasher.cpp#L35
    ///
    /// # See
    /// https://github.com/nemtech/catapult-server/blob/main/sdk/src/extensions/TransactionExtensions.cpp#L46
    ///
    /// # Inputs
    ///
    /// * `transaction_payload`: HexString Payload.
    /// * `generation_hash_buffer`: Network generation hash byte.
    ///
    /// # Returns
    ///
    /// A Transaction Payload hash.
    fn create_transaction_hash(
        transaction_payload: &str,
        generation_hash_buffer: &[u8],
    ) -> Result<H256> {
        // prepare hash
        let mut entity_hash = H256::zero();

        let transaction_bytes = hex_decode(transaction_payload);

        // read transaction type
        static TYPE_IDX: usize = Transaction::TYPE_INDEX;

        let mut type_bytes = (&transaction_bytes[TYPE_IDX..TYPE_IDX + 2]).to_vec(); // REVERSED
        type_bytes.reverse();
        let entity_type: TransactionType =
            TransactionType::try_from(u16::from_str_radix(hex::encode(&type_bytes).as_str(), 16)?)?;

        let is_aggregate_transaction = [
            TransactionType::AggregateBonded,
            TransactionType::AggregateComplete,
        ]
            .iter()
            .take_while(|x| **x == entity_type)
            .collect::<Vec<&TransactionType>>();

        // add full signature
        let signature = transaction_bytes[8..8 + 64].as_ref();

        // add public key to match sign/verify behavior (32 bytes)
        let public_key = transaction_bytes[8 + 64..8 + 64 + 32].as_ref();

        // add transaction data without header (EntityDataBuffer)
        // @link https://github.com/nemtech/catapult-server/blob/main/src/catapult/model/EntityHasher.cpp#L30
        let mut transaction_body = transaction_bytes[Self::HEADER_SIZE..].as_ref();

        // in case of aggregate transactions, we hash only the merkle transaction hash.
        if !is_aggregate_transaction.is_empty() {
            transaction_body = &transaction_bytes[Self::HEADER_SIZE..Self::BODY_INDEX + 32];
        }

        // concatenate binary hash parts
        // layout: `signature_R || signerPublicKey || generationHash || EntityDataBuffer`
        let mut entity_hash_bytes = Vec::with_capacity(
            signature.len()
                + public_key.len()
                + generation_hash_buffer.len()
                + transaction_body.len(),
        );
        entity_hash_bytes.extend_from_slice(signature);
        entity_hash_bytes.extend_from_slice(public_key);
        entity_hash_bytes.extend_from_slice(generation_hash_buffer);
        entity_hash_bytes.extend_from_slice(transaction_body);

        // 6) create SHA3 hash of transaction data
        // Note: Transaction hashing *always* uses SHA3
        let sha3_hash = Sha3_256::digest(entity_hash_bytes.as_slice());
        entity_hash.assign_from_slice(&sha3_hash.as_slice()[0..32]);

        Ok(entity_hash)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    const KNOWN_PAYLOAD: &str = "970000000000000075DAC796D500CEFDFBD582BC6E0580401FE6DB02FBEA93673DF47844246CDEA93715EB700F295A459E59D96A2BC6B7E36C79016A96B9FA387E8B8937342FE30C6BE37B726EEE24C4B0E3C943E09A44691553759A89E92C4A84BBC4AD9AF5D49C0000000001984E4140420F0000000000E4B580B11A000000A0860100000000002AD8FC018D9A49E100056576696173";
    const KNOWN_AGGREGATE_PAYLOAD: &str = "0801000000000000AC1F3E0EE2C16F465CDC2E091DC44D6EB55F7FE3988A5F21309DF479BE6D3F0033E155695FB1133EA0EA64A67C1EDC2B430CFAF9722AF36BAE84DBDB1C8F1509C2F93346E27CE6AD1A9F8F5E3066F8326593A406BDF357ACB041E2F9AB402EFE000000000180414200000000000000006BA50FB91A000000EA8F8301E7EDFD701F62E1DC1601ABDE22E5FCD11C9C7E7A01B87F8DFB6B62B060000000000000005D00000000000000C2F93346E27CE6AD1A9F8F5E3066F8326593A406BDF357ACB041E2F9AB402EFE0000000001A854419050B9837EFAB4BBE8A4B9BB32D812F9885C00D8FC1650E142000D000000000000746573742D6D657373616765000000";

    // expected values
    lazy_static! {
        static ref KNOWN_HASH_SHA3: H256 =
            H256::from_str("F0F5A62A0863D45E832B50EFF4E2F68157268A5D1674EC1068D82EC5F88D950B")
                .unwrap();
        static ref GENERATION_HASH_BYTES: Vec<u8> =
            hex_decode("988C4CDCE4D188013C13DE7914C7FD4D626169EF256722F61C52EFBE06BD5A2C");
        static ref GENERATION_HASH_BYTES_MT: Vec<u8> =
            hex_decode("17FA4747F5014B50413CCF968749604D728D7065DC504291EEE556899A534CBB");
    }

    #[test]
    fn create_different_hash_given_different_signatures() {
        let hash1 =
            Transaction::create_transaction_hash(&KNOWN_PAYLOAD, &GENERATION_HASH_BYTES).unwrap();

        // modify signature part of the payload ; this must affect produced hash
        let tampered_sig = (&KNOWN_PAYLOAD[0..16]).to_string() + "12" + &KNOWN_PAYLOAD[18..];
        let hash2 = Transaction::create_transaction_hash(
            &tampered_sig, // replaced two first bytes of signature
            &GENERATION_HASH_BYTES,
        )
            .unwrap();

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn create_different_hash_given_different_signer_public_key() {
        let hash1 =
            Transaction::create_transaction_hash(&KNOWN_PAYLOAD, &GENERATION_HASH_BYTES).unwrap();

        // modify signer public key part of the payload ; this must affect produced hash
        let tampered_signer =
            (&KNOWN_PAYLOAD[0..16 + 128]).to_string() + "12" + &KNOWN_PAYLOAD[16 + 128 + 2..];
        let hash2 = Transaction::create_transaction_hash(
            &tampered_signer, // replaced two first bytes of signature
            &GENERATION_HASH_BYTES,
        )
            .unwrap();

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn create_different_hash_given_different_generation_hash() {
        let hash1 =
            Transaction::create_transaction_hash(&KNOWN_PAYLOAD, &GENERATION_HASH_BYTES).unwrap();

        let hash2 = Transaction::create_transaction_hash(
            &KNOWN_PAYLOAD,
            &GENERATION_HASH_BYTES_MT, // uses different generation hash
        )
            .unwrap();

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn create_different_hash_given_different_different_transaction_body() {
        let hash1 =
            Transaction::create_transaction_hash(&KNOWN_PAYLOAD, &GENERATION_HASH_BYTES).unwrap();

        // modify "transaction body" part of payload ; this must affect produced transaction hash
        let tampered_body = (&KNOWN_AGGREGATE_PAYLOAD[0..Transaction::BODY_INDEX * 2]).to_string()
            + "12"
            + &KNOWN_AGGREGATE_PAYLOAD[Transaction::BODY_INDEX * 2 + 2..];
        let hash2 = Transaction::create_transaction_hash(
            &tampered_body,
            &GENERATION_HASH_BYTES_MT, // uses different generation hash
        )
            .unwrap();

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn create_same_hash_given_same_payloads() {
        let hash1 =
            Transaction::create_transaction_hash(&KNOWN_PAYLOAD, &GENERATION_HASH_BYTES).unwrap();

        let hash2 =
            Transaction::create_transaction_hash(&KNOWN_PAYLOAD, &GENERATION_HASH_BYTES).unwrap();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn create_correct_sha3_transaction_hash_given_network_type_private_or_private_test() {
        let hash1 =
            Transaction::create_transaction_hash(&KNOWN_PAYLOAD, &GENERATION_HASH_BYTES).unwrap();
        let hash2 =
            Transaction::create_transaction_hash(&KNOWN_PAYLOAD, &GENERATION_HASH_BYTES).unwrap();

        assert_eq!(hash1.to_fixed_bytes(), KNOWN_HASH_SHA3.to_fixed_bytes());
        assert_eq!(hash2.to_fixed_bytes(), KNOWN_HASH_SHA3.to_fixed_bytes());
    }

    #[test]
    fn hash_only_merkle_transaction_hash_for_aggregate_transactions() {
        let hash1 =
            Transaction::create_transaction_hash(&KNOWN_AGGREGATE_PAYLOAD, &GENERATION_HASH_BYTES)
                .unwrap();

        // modify end of payload ; this must not affect produced transaction hash
        // this test is valid only for Aggregate Transactions
        let tampered_size = "12".to_string() + &KNOWN_AGGREGATE_PAYLOAD[2..];
        let hash_tampered_body = Transaction::create_transaction_hash(
            &tampered_size, // replace in size (header change should not affect hash)
            &GENERATION_HASH_BYTES,
        )
            .unwrap();

        // modify "merkle hash" part of payload ; this must affect produced transaction hash
        let tampered_payload = (&KNOWN_AGGREGATE_PAYLOAD[0..Transaction::BODY_INDEX * 2])
            .to_string()
            + "12"
            + &KNOWN_AGGREGATE_PAYLOAD[Transaction::BODY_INDEX * 2 + 2..];

        let hash_tampered_merkle = Transaction::create_transaction_hash(
            &tampered_payload, // replace in merkle hash (will affect hash)
            &GENERATION_HASH_BYTES,
        )
            .unwrap();

        assert_eq!(hash1.to_fixed_bytes(), hash_tampered_body.to_fixed_bytes());
        assert_ne!(
            hash1.to_fixed_bytes(),
            hash_tampered_merkle.to_fixed_bytes()
        );
    }
}
