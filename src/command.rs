use crate::github::{IssueComment, GithubClient, MergeMethod};

lazy_static! {
    static ref GITHUB_CLIENT: GithubClient = GithubClient::new();
}

#[derive(Debug)]
pub enum Command {
    Hotfix,
    Release,
    Ping,
    Merge,
    Noop,
}

impl From<&str> for Command {
    fn from(string: &str) -> Self {
        match string {
            "!hotfix" => Command::Hotfix,
            "!release" => Command::Release,
            "!merge" => Command::Merge,
            "!ping" => Command::Ping,
            _ => Command::Noop,
        }
    }
}

impl Command {
    pub fn execute(&self, issue_comment: &IssueComment) {
        match self {
            Command::Hotfix => self.execute_hotfix(&issue_comment),
            Command::Release => self.execute_release(&issue_comment),
            Command::Ping => self.execute_ping(&issue_comment),
            Command::Merge => self.execute_merge(&issue_comment),
            Command::Noop => {},
        }
    }

    fn execute_hotfix(&self, issue_comment: &IssueComment) {
        let body = json!({
            "title": "🤖 Hotfix back-merge",
            "head": "master",
            "base": "development",
            "maintainer_can_modify": true,
            "body": "Beep boop, your wish is my command."
        });

        GITHUB_CLIENT.create_pull_request(
            issue_comment.repository_full_name(),
            body.to_string()
        ).unwrap();
    }

    fn execute_release(&self, issue_comment: &IssueComment) {
        let body = json!({
            "title": "🤖 Release",
            "head": "development",
            "base": "master",
            "maintainer_can_modify": true,
            "body": "Beep boop, your wish is my command."
        });

        GITHUB_CLIENT.create_pull_request(
            issue_comment.repository_full_name(),
            body.to_string()
        ).unwrap();
    }

    fn execute_ping(&self, issue_comment: &IssueComment) {
        let body = json!({
            "body": "pong"
        });

        GITHUB_CLIENT.create_comment(
            issue_comment.repository_full_name(),
            issue_comment.issue_number(),
            body.to_string()
        ).unwrap();
    }

    fn execute_merge(&self, issue_comment: &IssueComment) {
        let pull_request = GITHUB_CLIENT.pull_request_info(
            issue_comment.repository_full_name(),
            issue_comment.issue_number(),
        );

        let mut merge_method = MergeMethod::Squash;

        if pull_request.is_release() || pull_request.is_back_merge(){
            merge_method = MergeMethod::Merge;
        }

        if pull_request.is_hotfix() {
            merge_method = MergeMethod::Squash;
        }

        let body = json!({
            "merge_method": Into::<&str>::into(merge_method)
        });

        GITHUB_CLIENT.merge_pull_request(
            issue_comment.repository_full_name(),
            issue_comment.issue_number(),
            body.to_string(),
        ).unwrap();
    }
}
