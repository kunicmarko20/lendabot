#[derive(Deserialize, Debug)]
pub struct Commit {
    pub(in crate::github) message: String,
}
