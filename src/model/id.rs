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

use crate::Uint64;

/// An `trait` is used to define mosaicIds and namespaceIds
#[typetag::serde]
pub trait Id: Send + Sync
{
    fn to_uint64(&self) -> Uint64;

    fn box_clone(&self) -> Box<dyn Id>;
}

impl Clone for Box<dyn Id + 'static> {
    fn clone(&self) -> Box<dyn Id + 'static> {
        self.box_clone()
    }
}

impl<'a> PartialEq for &'a dyn Id {
    fn eq(&self, other: &Self) -> bool {
        self.to_uint64() == other.to_uint64()
    }
}

impl<'a> PartialEq for Box<dyn Id + 'static> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl fmt::Display for dyn Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_uint64().to_hex())
    }
}

impl fmt::Debug for dyn Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_uint64().to_hex())
    }
}