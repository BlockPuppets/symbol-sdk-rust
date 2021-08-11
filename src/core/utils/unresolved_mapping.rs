/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use crate::account::{Address, UnresolvedAddress};
use crate::mosaic::{MosaicId, UnresolvedMosaicId};
use crate::namespace::NamespaceId;
use crate::{hex_decode, is_hex};
use anyhow::{ensure, Result};

/// Map unresolved mosaic hex string to MosaicId or NamespaceId.
pub fn to_unresolved_mosaic(mosaic_id: &str) -> Result<Box<dyn UnresolvedMosaicId>> {
    ensure!(
        is_hex(mosaic_id),
        "Input string is not in valid hexadecimal notation."
    );

    let bytes = hex_decode(mosaic_id);
    let byte0 = &bytes[0];

    // if most significant bit of byte 0 is set, then we have a namespaceId
    if (byte0 & 128) == 128 {
        Ok(Box::new(NamespaceId::from_hex(mosaic_id)?))
    } else {
        // most significant bit of byte 0 is not set => mosaicId
        Ok(Box::new(MosaicId::from_hex(mosaic_id)?))
    }
}

/// Map unresolved address string to Address or NamespaceId.
pub fn to_unresolved_address(address: &str) -> Result<Box<dyn UnresolvedAddress>> {
    ensure!(
        is_hex(address),
        "Input string is not in valid hexadecimal notation."
    );

    let bytes = hex_decode(&address[0..2]);
    let byte0 = &bytes[0];

    // If bit 0 of byte 0 is not set (like in 0x90), then it is a regular address.
    // Else (e.g. 0x91) it represents a namespace id which starts at byte 1.
    if (byte0 & 16) == 16 {
        // namespaceId encoded hexadecimal notation provided
        // only 8 bytes are relevant to resolve the NamespaceId
        let relevant_part = &address[2..16];
        let mut relevant_part_vec = hex_decode(relevant_part);
        relevant_part_vec.reverse();

        let higher: u32 = u32::from_str_radix(&hex::encode(&relevant_part_vec)[0..8], 16)?;
        let lower: u32 = u32::from_str_radix(&hex::encode(relevant_part_vec)[8..], 16)?;

        Ok(Box::new(NamespaceId::from([higher, lower])))
    } else {
        // most significant bit of byte 0 is not set => mosaicId
        Ok(Box::new(Address::from_encoded(address)?))
    }
}
