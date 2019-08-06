mod back_merge;
mod release;
mod update_release;

pub(in crate::command) use self::back_merge::BackMerge;
pub(in crate::command) use self::release::Release;
pub(in crate::command) use self::update_release::UpdateRelease;
use crate::payload::SlashCommandPayload;
use lendabot::github::GithubClient;

type CommandResult = Result<(), String>;

#[derive(Debug)]
pub enum Command {
    Release,
    BackMerge,
    Noop,
}

impl From<&str> for Command {
    fn from(string: &str) -> Self {
        match string {
            "release" | "r" => Command::Release,
            "back-merge" | "bm" => Command::BackMerge,
            _ => Command::Noop,
        }
    }
}

impl Command {
    pub fn execute(
        &self,
        github_client: GithubClient,
        slash_command_payload: SlashCommandPayload,
    ) -> CommandResult {
        match self {
            Command::Release => return Release::execute(github_client, slash_command_payload),
            Command::BackMerge => return BackMerge::execute(github_client, slash_command_payload),
            Command::Noop => return Err("Command not found.".into()),
        }
    }
}
