#[derive(Deserialize, Debug)]
pub struct SlashCommandPayload {
    command: String,
    text: String,
}

impl SlashCommandPayload {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn command(&self) -> &str {
        &self.command
    }
}
