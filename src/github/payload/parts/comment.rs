#[derive(Deserialize, Debug)]
pub struct Comment {
    pub(in crate::github) body: String,
}
