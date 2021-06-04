#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page_number: i32,
    pub page_size: i32,
}


