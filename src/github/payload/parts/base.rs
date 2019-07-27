#[derive(Deserialize, Debug)]
pub struct Base {
    #[serde(rename = "ref")]
    pub(in crate::github) branch: String,
}
