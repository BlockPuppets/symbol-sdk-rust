/*
 * // Copyright 2021 BlockPuppets developers.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::io::Cursor;

use anyhow::{ensure, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use regex::Regex;
use sha3::{Digest, Sha3_256};

use crate::Uint64;

pub use self::namespace_id::*;

mod namespace_id;

fn namespace_id(namespace_name: &str) -> Result<Uint64> {
    let path = generate_namespace_path(namespace_name)?;
    Ok(path[path.len() - 1])
}

/// Parses a unified namespace name into a path.
///
fn generate_namespace_path(name: &str) -> Result<Vec<Uint64>> {
    ensure!(name.len() > 0, "having zero length");

    let mut parent_id = Uint64::default();
    let mut path: Vec<Uint64> = vec![];
    for part in name.split('.').collect::<Vec<&str>>() {
        parent_id = generate_namespace_id(parent_id, &valid_part_name(part)?)?;
        path.push(parent_id)
    }
    Ok(path)
}

pub(crate) fn generate_namespace_id(parent_id: Uint64, name: &str) -> Result<Uint64> {
    let mut hash = Sha3_256::default();

    let fixed_bytes = parent_id.to_le_bytes();

    hash.update(fixed_bytes);
    hash.update(name);

    let bytes = hash.finalize();

    let mut cursor = Cursor::new(bytes[..].as_ref());
    let value = cursor.read_u64::<LittleEndian>().unwrap();
    Ok(Uint64::from(value | 1 << 63))
}

fn valid_part_name(name: &str) -> Result<String> {
    let reg_valid_namespace: Regex = Regex::new(r"^[a-z0-9][a-z0-9-_]*$").unwrap();
    ensure!(
        reg_valid_namespace.is_match(name),
        format!("invalid part name {}", name)
    );
    Ok(name.to_string())
}

#[cfg(test)]
mod tests {
    use crate::namespace::{generate_namespace_id, generate_namespace_path};
    use crate::Uint64;

    const NEM_ID: [u32; 2] = [0x375ffa4b, 0x84b3552d];
    const NAMESPACE_BASE_ID: [u32; 2] = [0, 0];

    #[test]
    fn test_generates_correct_well_known_root_path() {
        let path = generate_namespace_path("nem").unwrap();
        assert_eq!(path.len(), 1);
        assert_eq!(path[0].to_dto(), NEM_ID);
    }

    #[test]
    fn test_supports_multi_level_namespaces() {
        let mut expected: Vec<Uint64> = vec![];
        expected.push(generate_namespace_id(Uint64::from(NAMESPACE_BASE_ID), "foo").unwrap());
        expected.push(generate_namespace_id(expected[0], "bar").unwrap());
        expected.push(generate_namespace_id(expected[1], "baz").unwrap());

        let path = generate_namespace_path("foo.bar.baz").unwrap();

        assert_eq!(path.len(), 3);
        assert_eq!(path, expected);
    }

    #[test]
    fn test_rejects_improper_qualified_names() {
        ["a:b:c", "a::b"].iter().for_each(|name| {
            assert!(generate_namespace_path(name).is_err());
        });
    }
}
