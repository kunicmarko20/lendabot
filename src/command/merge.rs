use super::GITHUB_CLIENT;
use crate::github::payload::IssueCommentEventPayload;
use crate::github::MergeMethod;

pub(super) struct Merge;

impl Merge {
    pub fn execute(issue_comment_payload: &IssueCommentEventPayload) {
        let pull_request = GITHUB_CLIENT.pull_request_info(
            issue_comment_payload.repository_full_name(),
            issue_comment_payload.issue_number(),
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
            issue_comment_payload.repository_full_name(),
            issue_comment_payload.issue_number(),
            body.to_string(),
        ).unwrap();
    }
}
