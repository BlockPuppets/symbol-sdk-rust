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

///  Custom trait for unresolved address
///
#[typetag::serde]
pub trait UnresolvedAddress: Sync + Send
where
    Self: fmt::Debug,
{
    fn recipient_to_string(&self) -> String;
    fn to_vec(&self) -> Vec<u8>;
    fn box_clone(&self) -> Box<dyn UnresolvedAddress>;
}

impl Clone for Box<dyn UnresolvedAddress + 'static> {
    fn clone(&self) -> Box<dyn UnresolvedAddress + 'static> {
        self.box_clone()
    }
}

impl<'a> PartialEq for &'a dyn UnresolvedAddress {
    fn eq(&self, other: &Self) -> bool {
        self.to_vec() == other.to_vec()
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