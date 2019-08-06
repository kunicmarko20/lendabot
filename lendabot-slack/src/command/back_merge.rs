use crate::command::CommandResult;
use crate::payload::SlashCommandPayload;
use lendabot::github::GithubClient;

pub struct BackMerge;

impl BackMerge {
    pub fn execute(
        github_client: GithubClient,
        slash_command_payload: SlashCommandPayload,
    ) -> CommandResult {
        let body = json!({
            "title": "🤖 Back-merge",
            "head": "master",
            "base": "development",
            "maintainer_can_modify": true,
        });

        let response = github_client.create_pull_request(
            slash_command_payload.repository_full_name(),
            body.to_string(),
        );

        if let Err(error_payload) = response {
            return Err(error_payload.to_string());
        }

        Ok(())
    }
}
