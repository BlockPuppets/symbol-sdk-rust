/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::fmt;

use crate::{account::Address, Uint64};

use super::{MosaicFlags, MosaicId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicInfo {
    /// Version.
    ///
    pub version: u16,

    /// The database record id.
    ///
    pub record_id: String,

    /// The mosaic id.
    ///
    pub id: MosaicId,

    /// The mosaic supply.
    ///
    pub supply: u64,

    /// The block height were mosaic was created.
    ///
    pub start_height: Uint64,

    /// The mosaic owner address.
    ///
    pub owner_address: Address,

    /// The mosaic revision.
    ///
    pub revision: u16,

    /// The mosaic flags.
    ///
    pub flags: MosaicFlags,

    /// Mosaic divisibility.
    ///
    pub divisibility: u8,

    /// Mosaic duration.
    ///
    pub duration: Uint64,
}

impl MosaicInfo {
    /// Is mosaic supply mutable
    ///
    pub fn is_supply_mutable(&self) -> bool {
        self.flags.supply_mutable
    }

    /// Is mosaic transferable
    ///
    pub fn is_transferable(&self) -> bool {
        self.flags.transferable
    }

    /// Is mosaic restrictable
    ///
    pub fn is_restrictable(&self) -> bool {
        self.flags.restrictable
    }

    /// Generate buffer.
    ///
    pub fn serialize() -> Vec<u8> {
        todo!()
    }
}

impl fmt::Display for MosaicInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
