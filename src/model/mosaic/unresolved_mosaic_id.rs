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

use crate::Uint64;
use anyhow::{anyhow, Result};
use std::any::Any;

/// An `trait` is used to define mosaicIds and namespaceIds
#[typetag::serde]
pub trait UnresolvedMosaicId: Send + Sync {
    fn to_uint64(&self) -> Uint64;
    fn box_clone(&self) -> Box<dyn UnresolvedMosaicId>;
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl dyn UnresolvedMosaicId {
    fn as_bytes(&self) -> [u8; 8] {
        self.to_uint64().to_fixed_bytes()
    }
}

impl Clone for Box<dyn UnresolvedMosaicId + 'static> {
    fn clone(&self) -> Box<dyn UnresolvedMosaicId + 'static> {
        self.box_clone()
    }
}

impl<'a> PartialEq for &'a dyn UnresolvedMosaicId {
    fn eq(&self, other: &Self) -> bool {
        self.to_uint64() == other.to_uint64()
    }
}

impl<'a> PartialEq for Box<dyn UnresolvedMosaicId + 'static> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl fmt::Display for dyn UnresolvedMosaicId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_uint64().to_hex())
    }
}

impl fmt::Debug for dyn UnresolvedMosaicId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_uint64().to_hex())
    }
}

impl dyn UnresolvedMosaicId {
    /// Downcast a reference to this generic `UnresolvedAddress` to a specific type.
    ///
    /// # Panics
    ///
    /// Panics if the type is not `T`. In normal usage, you should know the
    /// specific type. In other cases, use `try_downcast_ref`.
    ///
    pub fn downcast_ref<T: 'static + UnresolvedMosaicId>(&self) -> &T {
        self.try_downcast_ref::<T>()
            .unwrap_or_else(|| panic!("downcast to wrong type; original `UnresolvedAddress` type"))
    }

    /// Downcast a reference to this generic `UnresolvedAddress` to a specific type.
    #[inline]
    pub fn try_downcast_ref<T: 'static + UnresolvedMosaicId>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    /// Downcast this generic `UnresolvedAddress` to a specific type.
    ///
    /// # Panics
    ///
    /// Panics if the `UnresolvedAddress` type is not `T`. In normal usage, you should know the
    /// specific type. In other cases, use `try_downcast`.
    ///
    pub fn downcast<T: 'static + UnresolvedMosaicId>(self: Box<Self>) -> Box<T> {
        self.try_downcast().unwrap_or_else(|err| panic!("{}", err))
    }

    /// Downcast this generic `UnresolvedAddress` to a specific type.
    #[inline]
    pub fn try_downcast<T: 'static + UnresolvedMosaicId>(self: Box<Self>) -> Result<Box<T>> {
        if self.as_ref().as_any().is::<T>() {
            Ok(self.into_any().downcast().unwrap())
        } else {
            Err(anyhow!(
                "downcast to wrong UnresolvedAddress type; original UnresolvedAddress type"
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::utils::unresolved_mapping;
    use crate::mosaic::MosaicId;
    use crate::namespace::NamespaceId;

    lazy_static! {
        pub static ref MOSAIC_ID: MosaicId = MosaicId::from_hex("11F4B1B3AC033DB5").unwrap();
        pub static ref NAMESPACE_ID: NamespaceId =
            NamespaceId::from_hex("9550CA3FC9B41FC5").unwrap();
    }

    #[test]
    fn test_can_map_hex_string_to_mosaic_id() {
        let unresolved = unresolved_mapping::to_unresolved_mosaic(&MOSAIC_ID.to_hex()).unwrap();
        assert_eq!(unresolved.clone().try_downcast::<MosaicId>().is_ok(), true);
        assert_eq!(unresolved.try_downcast::<NamespaceId>().is_ok(), false);
    }

    #[test]
    fn test_can_map_hex_string_to_namespace_id() {
        let unresolved = unresolved_mapping::to_unresolved_mosaic(&NAMESPACE_ID.to_hex()).unwrap();
        assert_eq!(unresolved.clone().try_downcast::<NamespaceId>().is_ok(), true);
        assert_eq!(unresolved.try_downcast::<MosaicId>().is_ok(), false);
    }

    #[test]
    #[should_panic(expected = "Input string is not in valid hexadecimal notation.")]
    fn test_should_panic_if_id_not_in_hex() {
        let _ = unresolved_mapping::to_unresolved_mosaic("test").unwrap();
    }
}
