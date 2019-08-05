use std::collections::HashMap;

pub struct SlashCommandPayload {
    command: String,
    repository: String,
}

impl SlashCommandPayload {
    pub fn repository_full_name(&self) -> &str {
        &self.repository
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn from_request_body(map: HashMap<String, String>) -> Self {
        let mut split = map.get("text").unwrap().splitn(2, ' ');

        SlashCommandPayload {
            command: split.next().expect("Command missing.").to_owned(),
            repository: split.next().expect("Repository missing.").to_owned(),
        }
    }
}
