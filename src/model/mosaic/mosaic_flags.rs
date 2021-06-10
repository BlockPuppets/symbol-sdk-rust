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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicFlags {
    /// The creator can choose between a definition that allows a mosaic supply change at a later point or an immutable supply.
    /// Allowed values for the property are "true" and "false". The default value is "false".
    ///
    pub supply_mutable: bool,

    /// The creator can choose if the mosaic definition should allow for transfers of the mosaic among accounts other than the creator.
    /// If the property 'transferable' is set to "false", only transfer transactions
    /// having the creator as sender or as recipient can transfer mosaics of that type.
    /// If set to "true" the mosaics can be transferred to and from arbitrary accounts.
    /// Allowed values for the property are thus "true" and "false". The default value is "true".
    ///
    pub transferable: bool,

    /// Not all the mosaics of a given network will be subject to mosaic restrictions. The feature will only affect
    /// those to which the issuer adds the "restrictable" property explicitly at the moment of its creation. This
    /// property appears disabled by default, as it is undesirable for autonomous tokens like the public network currency.
    ///
    pub restrictable: bool,
}

impl MosaicFlags {
    pub fn create(supply_mutable: bool, transferable: bool, restrictable: bool) -> MosaicFlags {
        Self {
            supply_mutable,
            transferable,
            restrictable,
        }
    }

    /// Get mosaic flag value in number
    ///
    pub fn get_value(&self) -> u8 {
        return (if self.supply_mutable { 1 } else { 0 })
            + (if self.transferable { 2 } else { 0 })
            + (if self.restrictable { 4 } else { 0 });
    }
}

impl From<u8> for MosaicFlags {
    fn from(flag: u8) -> Self {
        let binary_flags: String = "00".to_string() + &format!("{:b}", flag >> 0);
        let binary_flags = binary_flags[binary_flags.len() - 3..].as_bytes();

        Self {
            supply_mutable: binary_flags[2] as char == '1',
            transferable: binary_flags[1] as char == '1',
            restrictable: binary_flags[0] as char == '1',
        }
    }
}

impl fmt::Display for MosaicFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::mosaic::MosaicFlags;

    #[test]
    fn test_should_create_from_num_value() {
        let mosaic_flags = MosaicFlags::from(7);

        assert_eq!(mosaic_flags.supply_mutable, true);
        assert_eq!(mosaic_flags.transferable, true);
        assert_eq!(mosaic_flags.restrictable, true);
    }

    #[test]
    fn test_should_create_with_static_values() {
        let mosaic_flags = MosaicFlags::create(false, false, false);

        assert_eq!(mosaic_flags.supply_mutable, false);
        assert_eq!(mosaic_flags.transferable, false);
        assert_eq!(mosaic_flags.restrictable, false);
    }

    #[test]
    fn test_should_return_corredt_flags_value() {
        let mosaic_flags = MosaicFlags::create(false, false, false);
        assert_eq!(mosaic_flags.get_value(), 0);

        let mosaic_flags = MosaicFlags::create(true, false, false);
        assert_eq!(mosaic_flags.get_value(), 1);

        let mosaic_flags = MosaicFlags::create(false, true, false);
        assert_eq!(mosaic_flags.get_value(), 2);

        let mosaic_flags = MosaicFlags::create(false, false, true);
        assert_eq!(mosaic_flags.get_value(), 4);

        let mosaic_flags = MosaicFlags::create(true, true, true);
        assert_eq!(mosaic_flags.get_value(), 7);

        let mosaic_flags = MosaicFlags::create(true, false, true);
        assert_eq!(mosaic_flags.get_value(), 5);

        let mosaic_flags = MosaicFlags::create(true, true, false);
        assert_eq!(mosaic_flags.get_value(), 3);
    }
}
