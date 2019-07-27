use super::{Base, Head};

#[derive(Deserialize, Debug)]
pub struct PullRequest {
    pub(in crate::github) base: Base,
    pub(in crate::github) head: Head,
    pub(in crate::github) merged: bool,
}
