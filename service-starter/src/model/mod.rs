pub mod doo;
pub mod dto;
pub mod po;

#[derive(Serialize, Deserialize, Debug)]
pub struct PageList<T> {
    pub total: u32,
    pub page: u32,
    #[serde(rename = "item")]
    pub items_per_page: u16,
    #[serde(rename = "list")]
    pub data: Vec<T>,
}
