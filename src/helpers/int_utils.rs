/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::num::ParseIntError;

// Determines the base of the number literal, depending on the prefix
fn determine_num_text_and_base(s: &str) -> (&str, u32) {
    match s.strip_prefix("0x") {
        Some(s_hex) => (s_hex, 16),
        None => (s, 10),
    }
}

// Parse a u64 from a decimal or hex encoding
pub fn parse_u64(s: &str) -> Result<u64, ParseIntError> {
    let (txt, base) = determine_num_text_and_base(s);
    u64::from_str_radix(txt, base)
}