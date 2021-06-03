use std::fmt;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum BlockOrderBy {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "height")]
    Height,
}

impl fmt::Display for BlockOrderBy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}


