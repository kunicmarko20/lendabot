use super::parts::{Base, Head};

#[derive(Deserialize, Debug)]
pub struct PullRequestPayload {
    number: u64,
    title: String,
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

    pub fn pull_request_number(&self) -> u64 {
        self.number
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}
