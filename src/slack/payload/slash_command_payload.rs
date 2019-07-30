#[derive(Deserialize, Debug)]
pub struct SlashCommandPayload {
    command: String,
    text: String,
}

impl SlashCommandPayload {
    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn command(&self) -> &String {
        &self.command
    }
}
