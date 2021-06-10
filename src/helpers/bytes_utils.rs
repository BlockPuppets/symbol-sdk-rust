/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use rand::RngCore;

pub fn random_bytes<const COUNT: usize>() -> [u8; COUNT] {
    let mut rng = rand::thread_rng();
    let mut buf = [0u8; COUNT];
    rng.try_fill_bytes(&mut buf).unwrap();
    buf
}