use super::parts::Commit;

#[derive(Deserialize, Debug)]
pub struct PullRequestCommitPayload {
    commit: Commit,
}

impl PullRequestCommitPayload {
    pub fn commit_message(&self) -> &String {
        &self.commit.message
    }
}
