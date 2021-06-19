use crate::account::Address;
use crate::model_dto::MosaicDto;
use crate::mosaic::{MosaicFlags, MosaicId, MosaicInfo};
use anyhow::Result;
use std::str::FromStr;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MosaicInfoDto {
    /// Internal resource identifier.
    pub id: String,
    pub mosaic: MosaicDto,
}

impl MosaicInfoDto {
    pub fn to_compact(&self) -> Result<MosaicInfo> {
        let dto = self.mosaic.clone();
        Ok(MosaicInfo {
            version: dto.version,
            record_id: self.id.clone(),
            id: MosaicId::from_hex(&dto.id)?,
            supply: u64::from_str(&dto.supply)?,
            start_height: u64::from_str(&dto.start_height)?.into(),
            owner_address: Address::from_encoded(dto.owner_address)?,
            revision: dto.revision,
            flags: MosaicFlags::from(dto.flags),
            divisibility: dto.divisibility,
            duration: u64::from_str(&dto.duration)?.into(),
        })
    }
}
