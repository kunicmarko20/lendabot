mod back_merge;
mod release;
mod update_release;

pub(in crate::command) use self::back_merge::BackMerge;
pub(in crate::command) use self::release::Release;
pub(in crate::command) use self::update_release::UpdateRelease;
use lendabot::github::GithubClient;

use crate::payload::SlashCommandPayload;

lazy_static! {
    static ref GITHUB_CLIENT: GithubClient = GithubClient::default();
}

#[derive(Debug)]
pub enum Command {
    Release,
    BackMerge,
    Noop,
}

impl From<&str> for Command {
    fn from(string: &str) -> Self {
        match string {
            "/release" => Command::Release,
            "/back-merge" => Command::BackMerge,
            _ => Command::Noop,
        }
    }
}

impl Command {
    pub fn execute(&self, slash_command_payload: SlashCommandPayload) {
        match self {
            Command::Release => Release::execute(slash_command_payload),
            Command::BackMerge => BackMerge::execute(slash_command_payload),
            Command::Noop => {}
        }
    }
}
