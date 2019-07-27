#[derive(Deserialize, Debug)]
pub struct Issue {
    pub(in crate::github) number: u64,
    pub(in crate::github) html_url: String,
}
