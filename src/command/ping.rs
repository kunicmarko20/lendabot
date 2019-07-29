use super::GITHUB_CLIENT;
use crate::github::payload::IssueCommentEventPayload;

pub(super) struct Ping;

impl Ping {
    pub fn execute(issue_comment_payload: &IssueCommentEventPayload) {
        let body = json!({
            "body": "pong"
        });

        GITHUB_CLIENT
            .create_comment(
                issue_comment_payload.repository_full_name(),
                *issue_comment_payload.issue_number(),
                body.to_string(),
            )
            .unwrap();
    }
}
