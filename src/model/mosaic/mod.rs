/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};
use sha3::{Digest, Sha3_256};

use crate::H192;
use crate::account::Address;

pub use self::mosaic::*;
pub use self::mosaic_id::*;
pub use self::mosaic_nonce::*;

mod mosaic;
mod mosaic_id;
mod mosaic_nonce;

/// Generates a `MosaicId` given a `MosaicNonce` and a `Address`.
///
fn generate_mosaic_id(nonce: MosaicNonce, owner_address: Address<H192>) -> MosaicId {
    let mut hash = Sha3_256::default();

    hash.input(*nonce);
    hash.input(owner_address.address);

    let bytes = hash.result();

    let mut cursor = Cursor::new(bytes[..].as_ref());
    let value = cursor.read_u64::<LittleEndian>().unwrap();

    let lower = value as u32;
    let higher = (value >> 32) as u32;

    (lower, higher).into()
}
