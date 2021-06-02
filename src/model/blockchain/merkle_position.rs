#[derive(Debug, Deserialize, Serialize)]
#[repr(u8)]
pub enum MerklePosition {
    Left,
    Right,
}