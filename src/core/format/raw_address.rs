/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use base32::Alphabet::RFC4648;
use ripemd160::Ripemd160;
use sha3::{Digest, Sha3_256};

use crate::{H256, Uint64};
use crate::account::Address;
use crate::network::NetworkType;

pub fn public_key_to_address(public_key: H256, network_type: NetworkType) -> Vec<u8> {
    // step 1: sha3 hash of the public key
    let public_key_hash = sha3::Sha3_256::digest(public_key.as_bytes());

    // step 2: ripemd160 hash of (1)
    let ripemd_hash = Ripemd160::digest(public_key_hash.as_ref());

    // step 3: add network identifier byte in front of (2)
    let mut decoded_address = Vec::with_capacity(Address::LENGTH_IN_DECODED);
    decoded_address.push(network_type.value());
    decoded_address.append(&mut ripemd_hash.to_vec());

    // step 4: concatenate (3) and the checksum of (3)
    let hash = sha3::Sha3_256::digest(decoded_address[..21].as_ref());
    decoded_address.append(&mut hash[..Address::CHECKSUM_SIZE].to_vec());
    decoded_address
}

pub fn decode_base32(bytes: &mut [u8], data: &str) {
    let length = bytes.len();
    let add_decode = base32::decode(RFC4648 { padding: true }, data).unwrap();
    bytes.copy_from_slice(&add_decode[..length])
}

pub fn encode_base32(data: &[u8]) -> String {
    let mut encode_address = base32::encode(RFC4648 { padding: true }, data);
    encode_address.truncate(Address::LENGTH_IN_BASE32);
    encode_address.to_uppercase()
}

pub fn raw_prettify(address: &str) -> String {
    let mut res: String = String::new();
    for i in 0..6 {
        res += &address[i * 6..i * 6 + 6];
        res.push('-');
    }

    res += &address[address.len() - Address::CHECKSUM_SIZE..];
    res
}

pub fn is_valid_address(decoded: &[u8], sizes_decoded: usize, checksum_size: usize) -> bool {
    if sizes_decoded != decoded.len() {
        return false;
    }

    let checksum_begin = sizes_decoded - checksum_size;
    let hash = Sha3_256::digest(&decoded[..checksum_begin]);
    let mut checksum = Vec::with_capacity(checksum_size);
    checksum.append(&mut hash[..checksum_size].to_vec());
    checksum == &decoded[checksum_begin..]
}

/// Format a namespaceId *alias* into a valid recipient field value.
///
pub fn alias_to_recipient(namespace_id: Uint64, network_type: NetworkType) -> Vec<u8> {
    // 0x91 | namespaceId on 8 bytes | 15 bytes 0-pad = 24 bytes
    let mut padded: Vec<u8> = vec![];
    padded.push(network_type.value() | 0x01);
    namespace_id.to_le_bytes().reverse();
    padded.append(&mut namespace_id.to_le_bytes().to_vec());
    padded.append(&mut hex::decode("00".repeat(15)).unwrap());
    padded
}
