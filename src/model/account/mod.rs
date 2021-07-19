/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

//! The `account` module provides functions for creating `Address`, `PublicAccount` and `Account`.

pub use self::account::*;
pub use self::address::*;
pub use self::public_account::*;
pub use self::unresolved_address::*;

mod account;
mod address;
mod public_account;
mod unresolved_address;
