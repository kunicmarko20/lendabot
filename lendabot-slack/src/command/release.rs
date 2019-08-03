use crate::payload::SlashCommandPayload;

use super::UpdateRelease;
use super::GITHUB_CLIENT;

pub(super) struct Release;

impl Release {
    pub fn execute(slash_command_payload: SlashCommandPayload) {
        //Needs a check around repository name

        let body = json!({
            "title": "🤖 Release",
            "head": "development",
            "base": "master",
            "maintainer_can_modify": true,
        });

        let pull_request =
            GITHUB_CLIENT.create_pull_request(slash_command_payload.text(), body.to_string());

        UpdateRelease::execute(
            slash_command_payload.text(),
            pull_request.pull_request_number(),
        );
    }
}
