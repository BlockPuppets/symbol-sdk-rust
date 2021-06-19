#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MosaicDto {
    /// The version of the state
    pub version: u16,
    /// Mosaic identifier.
    pub id: String,
    /// Absolute amount. An amount of 123456789 (absolute) for a mosaic with divisibility 6 means 123.456789 (relative).
    pub supply: String,
    /// Height of the blockchain.
    #[serde(rename = "startHeight")]
    pub start_height: String,
    /// Address expressed in hexadecimal base.
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    /// A number that allows uint 32 values.
    pub revision: u16,
    /// - 0x00 (none) - No flags present.
    /// - 0x01 (supplyMutable) - Mosaic supports supply changes even when mosaic owner owns partial supply.
    /// - 0x02 (transferable) - Mosaic supports transfers between arbitrary accounts. When not set, mosaic can only be transferred to and from mosaic owner.
    /// - 0x04 (restrictable) - Mosaic supports custom restrictions configured by mosaic owner.
    pub flags: u8,
    /// Determines up to what decimal place the mosaic can be divided.
    /// Divisibility of 3 means that a mosaic can be divided into smallest parts of 0.001 mosaics.
    /// The divisibility must be in the range of 0 and 6.
    pub divisibility: u8,
    /// Duration expressed in number of blocks.
    pub duration: String,
}
