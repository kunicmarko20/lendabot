use crate::payload::IssueCommentEventPayload;
use lendabot::github::GithubClient;

pub(super) struct Ping;

impl Ping {
    pub fn execute(github_client: GithubClient, issue_comment_payload: IssueCommentEventPayload) {
        let body = json!({
            "body": "pong"
        });

        github_client
            .create_comment(
                issue_comment_payload.repository_full_name(),
                issue_comment_payload.issue_number(),
                body.to_string(),
            )
            .unwrap();
    }
}
