use super::GITHUB_CLIENT;
use crate::github::IssueComment;

pub(super) struct Ping;

impl Ping {
    pub fn execute(issue_comment: &IssueComment) {
        let body = json!({
            "body": "pong"
        });

        GITHUB_CLIENT.create_comment(
            issue_comment.repository_full_name(),
            issue_comment.issue_number(),
            body.to_string()
        ).unwrap();
    }
}
