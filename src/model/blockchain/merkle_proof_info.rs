use super::MerklePathItem;

/// The block merkle proof info
///
#[derive(Debug, Deserialize, Serialize)]
pub struct MerkleProofInfo {
    /// Array of merkle path items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merkle_path: Option<Vec<MerklePathItem>>,
}
