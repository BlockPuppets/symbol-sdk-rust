#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MosaicDto {
    /// Mosaic identifier.
    pub id: String,
    /// Absolute amount. An amount of 123456789 (absolute) for a mosaic with divisibility 6 means 123.456789 (relative).
    pub amount: String,
}

