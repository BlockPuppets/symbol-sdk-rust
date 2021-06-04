//**************************************************************************************************
// Numbers
//**************************************************************************************************

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