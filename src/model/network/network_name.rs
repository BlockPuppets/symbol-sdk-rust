use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkName {
    /// Network name.
    pub name: String,
    /// A short text describing the network.
    pub description: String,
}

impl fmt::Display for NetworkName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
