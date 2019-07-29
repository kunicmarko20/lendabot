use super::parts::{PullRequest, Repository};

#[derive(Deserialize, Debug)]
pub struct PullRequestEventPayload {
    pull_request: PullRequest,
    repository: Repository,
}

impl PullRequestEventPayload {
    pub fn is_hotfix(&self) -> bool {
        &self.pull_request.base.branch == "master"
            && self.pull_request.head.branch.starts_with("hotfix")
    }

    pub fn is_merged(&self) -> bool {
        self.pull_request.merged
    }

    pub fn repository_full_name(&self) -> &String {
        &self.repository.full_name
    }
}
