use super::GITHUB_CLIENT;
use crate::payload::SlashCommandPayload;

pub struct BackMerge;

impl BackMerge {
    pub fn execute(slash_command_payload: SlashCommandPayload) {
        let body = json!({
            "title": "🤖 Back-merge",
            "head": "master",
            "base": "development",
            "maintainer_can_modify": true,
        });

        GITHUB_CLIENT.create_pull_request(slash_command_payload.text(), body.to_string());
    }
}
