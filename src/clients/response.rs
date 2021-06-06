#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SymbolError {
    pub code: String,
    pub message: String,
}

impl std::error::Error for SymbolError {}

impl std::fmt::Display for SymbolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SymbolResponse {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

impl SymbolResponse {
    pub fn new() -> Self {
        Self {
            result: None,
        }
    }
}
