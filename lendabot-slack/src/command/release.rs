use crate::payload::SlashCommandPayload;

use super::UpdateRelease;
use lendabot::github::GithubClient;

pub(super) struct Release;

impl Release {
    pub fn execute(github_client: GithubClient, slash_command_payload: SlashCommandPayload) {
        //Needs a check around repository name

        let body = json!({
            "title": "🤖 Release",
            "head": "development",
            "base": "master",
            "maintainer_can_modify": true,
        });

        let pull_request = github_client.create_pull_request(
            slash_command_payload.repository_full_name(),
            body.to_string(),
        );

        UpdateRelease::execute(
            github_client,
            slash_command_payload.repository_full_name(),
            pull_request.pull_request_number(),
        );
    }
}
