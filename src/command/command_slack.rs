use crate::command::Release;
use crate::slack::payload::SlashCommandPayload;

#[derive(Debug)]
pub enum Command {
    Release,
    Noop,
}

impl From<&str> for Command {
    fn from(string: &str) -> Self {
        match string {
            "/release" => Command::Release,
            _ => Command::Noop,
        }
    }
}

impl Command {
    pub fn execute(&self, slash_command_payload: &SlashCommandPayload) {
        match self {
            Command::Release => Release::execute(&slash_command_payload.text()),
            Command::Noop => {}
        }
    }
}
