use super::GITHUB_CLIENT;
use crate::payload::IssueCommentEventPayload;
use lendabot::github::MergeMethod;

pub(super) struct Merge;

impl Merge {
    pub fn execute(issue_comment_payload: IssueCommentEventPayload) {
        let pull_request = GITHUB_CLIENT.pull_request_info(
            issue_comment_payload.repository_full_name(),
            issue_comment_payload.issue_number(),
        );

        let body = if pull_request.is_release() || pull_request.is_back_merge() {
            json!({
                "merge_method": Into::<&str>::into(MergeMethod::Merge),
            })
        } else {
            json!({
                "commit_title": issue_comment_payload.issue_title(),
                "merge_method": Into::<&str>::into(MergeMethod::Squash),
            })
        };

        let response = GITHUB_CLIENT
            .merge_pull_request(
                issue_comment_payload.repository_full_name(),
                issue_comment_payload.issue_number(),
                body.to_string(),
            )
            .unwrap();

        if response.status() == 405 {
            let body = json!({
                "body": "Unable to merge, did you try to approve pull request?"
            });

            GITHUB_CLIENT
                .create_comment(
                    issue_comment_payload.repository_full_name(),
                    issue_comment_payload.issue_number(),
                    body.to_string(),
                )
                .unwrap();
        }
    }
}
