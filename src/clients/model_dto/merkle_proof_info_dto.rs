use super::MerklePathItemDto;

#[serde(rename_all = "camelCase")]
#[derive(Clone, Serialize, Deserialize)]
pub struct MerkleProofInfoDto {
    /// List of complementary merkle path items needed to recalculate the merkle root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merkle_path: Option<Vec<MerklePathItemDto>>,
}