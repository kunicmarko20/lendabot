#[derive(Deserialize, Debug)]
pub struct Error {
    resource: String,
    code: String,
    pub(in crate::github) message: String,
}
