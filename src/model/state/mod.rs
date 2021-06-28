/*
 * // Copyright 2021 BlockPuppets developers.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

pub use self::merkle_tree::*;
pub use self::merkle_tree_branch::*;
pub use self::merkle_tree_branch_link::*;
pub use self::merkle_tree_leaf::*;
pub use self::merkle_tree_node_type::*;
pub use self::merkle_tree_parser::*;
pub use self::state_merkle_proof::*;

mod merkle_tree;
mod merkle_tree_branch;
mod merkle_tree_branch_link;
mod merkle_tree_leaf;
mod merkle_tree_node_type;
mod merkle_tree_parser;
mod state_merkle_proof;

#[cfg(test)]
mod tests {
    use hex::ToHex;
    use serde_json::Value;

    use crate::state::{MerkleTree, MerkleTreeNodeType};

    const MERKLE_STATE_DTO: &str = r#"
    {
        "raw": "00000082F44AB1A5C28DC667A4AB0C1CF4FCC1D872E4FCBAB6F23F930DD5178829BCED41B00876C8227C1ED98E870FACF99B53F5AD191D4DE0BC622EE5632D3ADB5C39D0FF3F785F565CC8239D316CF31138B168E7ED4B0D75459C487E0F5851A384053A5E0053C7CA6CCA284CCDB302A8A3CBBF4E60D18BC5D3CA83626DD918E8DF8F860E67",
        "tree": [
            {
                "type": 0,
                "path": "",
                "encodedPath": "00",
                "nibbleCount": 0,
                "linkMask": "8200",
                "links": [
                    {
                        "bit": "9",
                        "link": "F44AB1A5C28DC667A4AB0C1CF4FCC1D872E4FCBAB6F23F930DD5178829BCED41"
                    },
                    {
                        "bit": "F",
                        "link": "B00876C8227C1ED98E870FACF99B53F5AD191D4DE0BC622EE5632D3ADB5C39D0"
                    }
                ],
                "branchHash": "B982D5394C24C3D59D2D8679A2EABD4C25D8CAB62A8BC16B43C9FC9344682D44"
            },
            {
                "type": 255,
                "path": "785F565CC8239D316CF31138B168E7ED4B0D75459C487E0F5851A384053A5E00",
                "encodedPath": "3785F565CC8239D316CF31138B168E7ED4B0D75459C487E0F5851A384053A5E0",
                "nibbleCount": 63,
                "value": "53C7CA6CCA284CCDB302A8A3CBBF4E60D18BC5D3CA83626DD918E8DF8F860E67",
                "leafHash": "B00876C8227C1ED98E870FACF99B53F5AD191D4DE0BC622EE5632D3ADB5C39D0"
            }
        ]
    }"#;

    #[test]
    fn test_from_raw() {
        let dto: Value = serde_json::from_str(MERKLE_STATE_DTO).unwrap();
        let tree = MerkleTree::from_raw(dto["raw"].as_str().unwrap()).unwrap();
        assert_eq!(tree.branches.len(), 1);
        assert_eq!(
            tree.branches[0].branch_hash.encode_hex_upper::<String>(),
            dto["tree"][0]["branchHash"].as_str().unwrap()
        );
        assert_eq!(tree.branches[0].encoded_path, dto["tree"][0]["encodedPath"]);
        assert_eq!(tree.branches[0].path, dto["tree"][0]["path"]);
        assert_eq!(tree.branches[0].link_mask, dto["tree"][0]["linkMask"]);
        assert!(tree.leaf.is_some());

        if let Some(leaf) = tree.leaf {
            assert_eq!(leaf.r#type, MerkleTreeNodeType::Leaf);
            assert_eq!(
                leaf.leaf_hash.encode_hex_upper::<String>(),
                dto["tree"][1]["leafHash"].as_str().unwrap()
            );
            assert_eq!(
                leaf.encoded_path,
                dto["tree"][1]["encodedPath"].as_str().unwrap()
            );
            assert_eq!(
                leaf.path.to_uppercase(),
                dto["tree"][1]["path"].as_str().unwrap()
            );
            assert_eq!(
                leaf.value.encode_hex_upper::<String>(),
                dto["tree"][1]["value"].as_str().unwrap()
            );
        }
    }
}
