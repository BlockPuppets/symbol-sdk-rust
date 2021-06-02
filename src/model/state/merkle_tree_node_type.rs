#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
#[repr(u8)]
pub enum MerkleTreeNodeType {
    Branch = 0x00,
    Leaf = 0xff,
}
