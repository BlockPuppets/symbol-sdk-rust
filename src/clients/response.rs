use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonRpcError {
    pub code: i16,
    pub message: String,
    pub data: Option<Value>,
}

impl std::error::Error for JsonRpcError {}

impl std::fmt::Display for JsonRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct JsonResponse {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

impl JsonResponse {
    pub fn new() -> Self {
        Self {
            result: None,
        }
    }
}
