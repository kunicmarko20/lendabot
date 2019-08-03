use super::UpdateRelease;
use super::GITHUB_CLIENT;
use crate::payload::IssueCommentEventPayload;

pub(super) struct Release;

impl Release {
    pub fn execute(issue_comment_payload: IssueCommentEventPayload) {
        let body = json!({
            "title": "🤖 Release",
            "head": "development",
            "base": "master",
            "maintainer_can_modify": true,
        });

        GITHUB_CLIENT.create_pull_request(
            issue_comment_payload.repository_full_name(),
            body.to_string(),
        );

        UpdateRelease::execute(issue_comment_payload);
    }
}
