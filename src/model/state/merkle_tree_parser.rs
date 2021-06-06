/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::str::FromStr;

use anyhow::{bail, Result};
use hex::ToHex;
use sha3::{Digest, Sha3_256};

use crate::{H256, hex_decode, parse_u64};

use super::{
    MerkleTreeBranch, MerkleTreeBranchLink, MerkleTreeLeaf, MerkleTreeNodeType, MerkleTreeTrait,
};

pub struct MerkleTreeParser {}

impl MerkleTreeParser {
    /// Recursively parse raw tree
    ///
    pub fn parse_merkle_tree_from_raw(raw: &[u8]) -> Result<Vec<Box<dyn MerkleTreeTrait>>> {
        let mut merkle_tree: Vec<Box<dyn MerkleTreeTrait>> = vec![];

        if raw.is_empty() {
            return Ok(merkle_tree);
        }
        let marker = raw[0];
        let nibble_count = raw[1];
        let path_length = Self::get_path_length(nibble_count);
        let path = &raw[2..2 + path_length as usize];
        if Self::is_branch(marker) {
            let (less_branch, branch) =
                Self::parse_branch(&raw[2 + path_length..], path, nibble_count as usize);

            merkle_tree.push(Box::new(branch));
            merkle_tree.append(&mut Self::parse_merkle_tree_from_raw(&less_branch)?);

            Ok(merkle_tree)
        } else if Self::is_leaf(marker) {
            let (less_leaf, leaf) =
                Self::parse_leaf(&raw[2 + path_length..], path, nibble_count as usize);

            merkle_tree.push(Box::new(leaf));
            merkle_tree.append(&mut Self::parse_merkle_tree_from_raw(&less_leaf)?);

            Ok(merkle_tree)
        } else {
            bail!("{} is not a branch or a leaf!", hex::encode(raw));
        }
    }

    /// Decompose a bitmask to get number of bit's indices.
    ///
    fn get_bits_from_mask(mask: &mut [u8]) -> Vec<String> {
        mask.reverse();
        let int_value = parse_u64(&format!("0x{}", hex::encode(&mask))).unwrap();

        let mut index = 0_u32;
        let mut bits: Vec<String> = vec![];
        let mut i = 1;
        while i <= int_value {
            if 0 < (int_value & i) {
                bits.push(format!("{:0X}", index));
            }
            i *= 2;
            index += 1;
        }
        bits
    }

    /// Calculate path length from given nibbles count.
    ///
    fn get_path_length(nibble_count: u8) -> usize {
        // 1 nibble = 0.5 bytes.
        // Round up to the whole bytes
        (nibble_count as f64 / 2_f64).ceil() as usize
    }

    /// Is branch node.
    ///
    fn is_branch(marker: u8) -> bool {
        0 == marker
    }

    /// Is leaf node.
    ///
    fn is_leaf(marker: u8) -> bool {
        255 == marker
    }

    /// Parse branch tree node.
    ///
    fn parse_branch(
        offset_raw: &[u8],
        path: &[u8],
        nibble_count: usize,
    ) -> (Vec<u8>, MerkleTreeBranch) {
        let mut link_mask = [0u8; 2];
        link_mask.copy_from_slice(&offset_raw[0..2]);

        let bits = Self::get_bits_from_mask(&mut link_mask);
        let links_raw = &offset_raw[2..2 + 32 * bits.len()];
        let mut links: Vec<MerkleTreeBranchLink> = vec![];
        for (i, bit) in bits.iter().enumerate() {
            links.push(MerkleTreeBranchLink {
                bit: bit.to_uppercase(),
                link: H256::from_slice(&links_raw[i * 32..i * 32 + 32]),
            });
        }
        let encoded_path = Self::encode_path(path, nibble_count, false);

        let branch = MerkleTreeBranch {
            r#type: MerkleTreeNodeType::Branch,
            path: hex::encode(path),
            encoded_path: encoded_path.to_owned(),
            nibble_count,
            link_mask: hex::encode(link_mask),
            branch_hash: Self::get_branch_hash(encoded_path, &links),
            links,
        };

        let mut remaining = vec![];
        remaining.append(&mut offset_raw[2 + 32 * bits.len()..].to_vec());
        (remaining, branch)
    }

    /// Parse leaf tree node
    ///
    fn parse_leaf(
        offset_raw: &[u8],
        path: &[u8],
        nibble_count: usize,
    ) -> (Vec<u8>, MerkleTreeLeaf) {
        let value = H256::from_slice(&offset_raw[0..32]);
        let encoded_path = Self::encode_path(path, nibble_count, true);

        let leaf = MerkleTreeLeaf {
            r#type: MerkleTreeNodeType::Leaf,
            path: H256::from_slice(path),
            leaf_hash: Self::get_leaf_hash(encoded_path.to_owned(), value),
            encoded_path: H256::from_str(encoded_path.as_str()).unwrap(),
            nibble_count,
            value,
        };
        let mut raw_vec = vec![];
        raw_vec.copy_from_slice(&offset_raw[32..]);
        (raw_vec, leaf)
    }

    ///Encode path depends on node type and nibble count.
    ///
    fn encode_path(path: &[u8], nibble_count: usize, is_leaf: bool) -> String {
        let key_size = (nibble_count as f32 / 2_f32).floor() + 1_f32;

        let mut encoded_key = vec![0].repeat(key_size as usize);

        encoded_key[0] = if is_leaf { 0x20 } else { 0 }; // set leaf flag

        let mut i = 0;
        if 1 == nibble_count % 2 {
            // set odd flag and merge in first nibble
            encoded_key[0] = encoded_key[0] | 0x10 | Self::nibble_at(path, 0);
            i += 1;
        }

        while i < nibble_count {
            encoded_key[((i as f32 / 2_f32).floor() + 1_f32) as usize] =
                (Self::nibble_at(path, i) << 4) + Self::nibble_at(path, i + 1);
            i += 2;
        }

        hex::encode_upper(&encoded_key)
    }

    /// Get byte at given nibble index.
    ///
    fn nibble_at(path: &[u8], index: usize) -> u8 {
        let byte = path[(index as f32 / 2_f32).floor() as usize];
        if 0 == index % 2 {
            (byte & 0xf0) >> 4
        } else {
            byte & 0x0f
        }
    }

    /// Calculate branch hash. Hash(encoded_path + 16 links).
    ///
    fn get_branch_hash(encoded_path: String, links: &[MerkleTreeBranchLink]) -> H256 {
        let mut branch_links: [String; 16] = Default::default();
        branch_links.fill(format!("{:x}", H256::zero()));

        links.iter().for_each(|link| {
            let parse_bit = parse_u64(&format!("0x{}", link.bit)).unwrap();
            branch_links[parse_bit as usize] = link.link.encode_hex::<String>();
        });

        let mut hash = Sha3_256::new();

        hash.input(&hex_decode(&(encoded_path + &branch_links.join(""))));

        H256::from_slice(hash.result().as_slice())
    }

    /// Calculate leaf hash. Hash(encoded_path + leaf value).
    ///
    fn get_leaf_hash(encoded_path: String, leaf_value: H256) -> H256 {
        let mut hash = Sha3_256::new();
        hash.input(&hex_decode(&(encoded_path + &leaf_value.encode_hex::<String>())));

        H256::from_slice(hash.result().as_slice())
    }
}
