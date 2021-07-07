/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

/// The supply type. Supported supply types are:
/// 0: Increase in supply.
/// 1: Decrease in supply.
///
pub enum MosaicSupplyChangeAction {
    Decrease = 0x00,
    Increase = 0x01,
}