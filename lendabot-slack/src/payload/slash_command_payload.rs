use std::collections::HashMap;
use std::env;

lazy_static! {
    static ref ALLOWED_CHANNEL: String = env::var("APP_SLACK_ALLOWED_TRIGGER_CHANNEL")
        .expect("APP_SLACK_ALLOWED_TRIGGER_CHANNEL not set.");
}

pub struct SlashCommandPayload {
    command: String,
    channel_id: String,
    repository: String,
}

impl SlashCommandPayload {
    pub fn from_request_body(body: HashMap<String, String>) -> Self {
        let mut split = body.get("text").unwrap().splitn(2, ' ');

        SlashCommandPayload {
            command: split.next().expect("Command missing.").to_owned(),
            repository: split.next().expect("Repository missing.").to_owned(),
            channel_id: body.get("channel_id").unwrap().to_owned(),
        }
    }

    pub fn repository_full_name(&self) -> &str {
        &self.repository
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn has_permissions(&self) -> bool {
        self.channel_id == *ALLOWED_CHANNEL
    }
}
