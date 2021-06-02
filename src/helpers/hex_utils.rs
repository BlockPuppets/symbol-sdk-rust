/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use regex::Regex;

pub fn is_hex(input: &str) -> bool {
    if input.len() % 2 != 0 {
        return false;
    };

    let re = Regex::new(r"^[a-fA-F0-9]+$").unwrap();
    re.is_match(input)
}

pub fn hex_decode(data: &str) -> Vec<u8> {
    hex::decode(data)
        .map_err(|err| panic!("Failed to decode hex data {} : {}", data, err))
        .unwrap()
}

pub fn hex_to_utf8(hex: &str) -> String {
    let decode = hex_decode(hex);
    String::from_utf8(decode).unwrap()
}

// pub fn utf8_to_hex(txt: &str) -> String {
//     use core::fmt::Write;
//     let mut ret = String::with_capacity(2 * txt.len());
//     for ch in txt.as_bytes() {
//         write!(ret, "{:02x}", ch).expect("writing to string");
//     }
//     ret
// }