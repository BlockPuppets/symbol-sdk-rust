/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use ::std::mem::size_of;

use base32::Alphabet::RFC4648;
use ripemd160::Ripemd160;
use sha3::Digest;

use crate::H256;
use crate::network::NetworkType;

pub fn public_key_to_address<D: Digest, H>(
    public_key: H256,
    network_type: NetworkType,
    checksum_size: usize,
) -> Vec<u8> {
    // step 1: sha3 hash of the public key
    let public_key_hash = D::digest(public_key.as_bytes());

    // step 2: ripemd160 hash of (1)
    let ripemd_hash = Ripemd160::digest(&public_key_hash);

    // step 3: add network identifier byte in front of (2)
    let mut decoded_address = Vec::with_capacity(size_of::<H>());
    decoded_address.push(network_type.value());
    decoded_address.append(&mut ripemd_hash.to_vec());

    // step 4: concatenate (3) and the checksum of (3)
    let hash = D::digest(&decoded_address[..21]);
    decoded_address.append(&mut hash[..checksum_size].to_vec());
    decoded_address
}

pub fn decode_base32(bytes: &mut [u8], data: &str) {
    let length = bytes.len();
    let add_decode = base32::decode(RFC4648 { padding: true }, data).unwrap();
    bytes.copy_from_slice(&add_decode[..length])
}

pub fn encode_base32(data: &[u8], truncate_len: usize) -> String {
    let mut encode_address = base32::encode(RFC4648 { padding: true }, data);
    encode_address.truncate(truncate_len);
    encode_address.to_uppercase()
}

pub fn raw_prettify(address: &str, size_suffix: usize) -> String {
    let mut res: String = String::new();
    for i in 0..6 {
        res += &address[i * 6..i * 6 + 6];
        res.push('-');
    }

    res += &address[address.len() - size_suffix..];
    res
}