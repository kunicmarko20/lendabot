use crate::payload::IssueCommentEventPayload;
use lendabot::github::GithubClient;
use lendabot::github::MergeMethod;

pub(super) struct Merge;

impl Merge {
    pub fn execute(github_client: GithubClient, issue_comment_payload: IssueCommentEventPayload) {
        let pull_request = github_client.pull_request_info(
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

        let response = github_client
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

            github_client
                .create_comment(
                    issue_comment_payload.repository_full_name(),
                    issue_comment_payload.issue_number(),
                    body.to_string(),
                )
                .unwrap();
        }
    }
}
