/// Position relative to the proofHash being evaluated.
///
#[derive(Debug, Clone, Deserialize, Serialize)]
#[repr(u8)]
pub enum MerklePosition {
    Left,
    Right,
}