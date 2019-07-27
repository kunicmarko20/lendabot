use super::GITHUB_CLIENT;
use crate::github::IssueComment;

pub(super) struct Hotfix;

impl Hotfix {
    pub fn execute(issue_comment: &IssueComment) {
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
}
