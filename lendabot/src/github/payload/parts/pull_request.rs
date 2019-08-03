use super::{Base, Head};

#[derive(Deserialize, Debug)]
pub struct PullRequest {
    base: Base,
    head: Head,
    merged: bool,
}

impl PullRequest {
    pub fn base_branch(&self) -> &str {
        &self.base.branch
    }

    pub fn head_branch(&self) -> &str {
        &self.head.branch
    }

    pub fn merged(&self) -> bool {
        self.merged
    }
}
