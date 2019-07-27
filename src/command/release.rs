use super::GITHUB_CLIENT;
use crate::github::payload::IssueCommentEventPayload;

pub(super) struct Release;

impl Release {
    pub fn execute(issue_comment_payload: &IssueCommentEventPayload) {
        let body = json!({
            "title": "🤖 Release",
            "head": "development",
            "base": "master",
            "maintainer_can_modify": true,
            "body": "Beep boop, your wish is my command."
        });

        GITHUB_CLIENT.create_pull_request(
            issue_comment_payload.repository_full_name(),
            body.to_string()
        ).unwrap();
    }
}
