use super::parts::{Base, Head};

#[derive(Deserialize, Debug)]
pub struct PullRequestPayload {
    base: Base,
    head: Head,
}

impl PullRequestPayload {
    pub fn is_release(&self) -> bool {
        &self.base.branch == "master" && &self.head.branch == "development"
    }

    pub fn is_back_merge(&self) -> bool {
        &self.base.branch == "development" && &self.head.branch == "master"
    }

    pub fn is_hotfix(&self) -> bool {
        &self.base.branch == "master" && self.head.branch.starts_with("hotfix")
    }
}
