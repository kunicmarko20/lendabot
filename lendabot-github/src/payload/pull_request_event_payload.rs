use lendabot::github::payload::{PullRequest, Repository};

#[derive(Deserialize, Debug)]
pub struct PullRequestEventPayload {
    pull_request: PullRequest,
    repository: Repository,
}

impl PullRequestEventPayload {
    pub fn is_hotfix(&self) -> bool {
        self.pull_request.base_branch() == "master"
            && self.pull_request.head_branch().starts_with("hotfix")
    }

    pub fn is_merged(&self) -> bool {
        self.pull_request.merged()
    }

    pub fn repository_full_name(&self) -> &str {
        &self.repository.full_name()
    }
}
