#[derive(Deserialize, Debug)]
pub struct Repository {
    pub(in crate::github) full_name: String,
}
