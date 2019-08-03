use super::UpdateRelease;
use crate::payload::IssueCommentEventPayload;
use lendabot::github::GithubClient;

pub(super) struct Release;

impl Release {
    pub fn execute(github_client: GithubClient, issue_comment_payload: IssueCommentEventPayload) {
        let body = json!({
            "title": "🤖 Release",
            "head": "development",
            "base": "master",
            "maintainer_can_modify": true,
        });

        github_client.create_pull_request(
            issue_comment_payload.repository_full_name(),
            body.to_string(),
        );

        UpdateRelease::execute(github_client, issue_comment_payload);
    }
}
