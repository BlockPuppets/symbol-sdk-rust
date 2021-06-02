use anyhow::{anyhow, Result};

#[derive(Debug, Deserialize, Serialize)]
#[repr(u16)]
pub enum BlockType {
    NemesisBlock = 0x8043,
    NormalBlock = 0x8143,
    ImportanceBlock = 0x8243,
}

impl Default for BlockType {
    fn default() -> Self {
        BlockType::NormalBlock
    }
}

impl std::convert::TryFrom<u16> for BlockType {
    type Error = anyhow::Error;

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        use BlockType::*;
        match v {
            x if x == NemesisBlock as u16 => Ok(NemesisBlock),
            x if x == NormalBlock as u16 => Ok(NormalBlock),
            x if x == ImportanceBlock as u16 => Ok(ImportanceBlock),
            _ => Err(anyhow!("Invalid value blockType")),
        }
    }
}