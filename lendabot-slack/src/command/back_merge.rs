use crate::payload::SlashCommandPayload;
use lendabot::github::GithubClient;

pub struct BackMerge;

impl BackMerge {
    pub fn execute(github_client: GithubClient, slash_command_payload: SlashCommandPayload) {
        let body = json!({
            "title": "🤖 Back-merge",
            "head": "master",
            "base": "development",
            "maintainer_can_modify": true,
        });

        github_client.create_pull_request(slash_command_payload.text(), body.to_string());
    }
}
