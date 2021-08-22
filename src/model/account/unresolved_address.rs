/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::{any::Any, fmt};

use anyhow::{anyhow, Result};

use crate::network::NetworkType;

///  Custom trait for unresolved address
///
#[typetag::serde]
pub trait UnresolvedAddress: Sync + Send
    where
        Self: fmt::Debug,
{
    fn recipient_to_string(&self) -> String;
    fn unresolved_address_to_bytes(&self, network_type: NetworkType) -> Vec<u8>;
    fn box_clone(&self) -> Box<dyn UnresolvedAddress>;
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl Clone for Box<dyn UnresolvedAddress + 'static> {
    fn clone(&self) -> Box<dyn UnresolvedAddress + 'static> {
        self.box_clone()
    }
}

impl<'a> PartialEq for &'a dyn UnresolvedAddress {
    fn eq(&self, other: &Self) -> bool {
        self.recipient_to_string() == other.recipient_to_string()
    }
}

impl<'a> PartialEq for Box<dyn UnresolvedAddress + 'static> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl fmt::Display for dyn UnresolvedAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.recipient_to_string())
    }
}

impl dyn UnresolvedAddress {
    /// Downcast a reference to this generic `UnresolvedAddress` to a specific type.
    ///
    /// # Panics
    ///
    /// Panics if the type is not `T`. In normal usage, you should know the
    /// specific type. In other cases, use `try_downcast_ref`.
    ///
    pub fn downcast_ref<T: 'static + UnresolvedAddress>(&self) -> &T {
        self.try_downcast_ref::<T>()
            .unwrap_or_else(|| panic!("downcast to wrong UnresolvedAddress type; original `UnresolvedAddress` type"))
    }

    /// Downcast a reference to this generic `UnresolvedAddress` to a specific type.
    #[inline]
    pub fn try_downcast_ref<T: 'static + UnresolvedAddress>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    /// Downcast this generic `UnresolvedAddress` to a specific type.
    ///
    /// # Panics
    ///
    /// Panics if the `UnresolvedAddress` type is not `T`. In normal usage, you should know the
    /// specific type. In other cases, use `try_downcast`.
    ///
    pub fn downcast<T: 'static + UnresolvedAddress>(self: Box<Self>) -> Box<T> {
        self.try_downcast().unwrap_or_else(|err| panic!("{}", err))
    }

    /// Downcast this generic `UnresolvedAddress` to a specific type.
    #[inline]
    pub fn try_downcast<T: 'static + UnresolvedAddress>(self: Box<Self>) -> Result<Box<T>> {
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
    use hex::ToHex;

    use crate::account::Address;
    use crate::core::utils::unresolved_mapping;
    use crate::namespace::NamespaceId;

    lazy_static! {
        pub static ref NAMESPACE_ID: NamespaceId =
            NamespaceId::from_hex("9550CA3FC9B41FC5").unwrap();
        pub static ref ADDRESS: Address =
            Address::from_raw("VATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA35C4KNQ").unwrap();
    }

    #[test]
    fn test_can_map_hex_str_to_address() {
        let unresolved =
            unresolved_mapping::to_unresolved_address(&ADDRESS.address.encode_hex::<String>())
                .unwrap();
        assert_eq!(unresolved.clone().try_downcast::<Address>().is_ok(), true);
        assert_eq!(unresolved.try_downcast::<NamespaceId>().is_ok(), false);
    }

    #[test]
    fn test_can_map_hex_str_to_namespace_id() {
        let unresolved = unresolved_mapping::to_unresolved_address(&NAMESPACE_ID.to_hex()).unwrap();
        assert_eq!(
            unresolved.clone().try_downcast::<NamespaceId>().is_ok(),
            true
        );
        assert_eq!(unresolved.try_downcast::<Address>().is_ok(), false);
    }

    #[test]
    #[should_panic(expected = "Input string is not in valid hexadecimal notation.")]
    fn test_should_panic_if_id_not_in_hex() {
        let _ = unresolved_mapping::to_unresolved_address("test").unwrap();
    }
}
