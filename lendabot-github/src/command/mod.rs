mod back_merge;
mod merge;
mod ping;
mod release;
mod update_release;

pub use self::back_merge::BackMerge;

pub(in crate::command) use self::merge::Merge;
pub(in crate::command) use self::ping::Ping;

pub(in crate::command) use self::release::Release;
pub(in crate::command) use self::update_release::UpdateRelease;
use crate::payload::IssueCommentEventPayload;
use lendabot::github::GithubClient;

#[derive(Debug)]
pub enum Command {
    BackMerge,
    Release,
    UpdateRelease,
    Ping,
    Merge,
    Noop,
}

impl From<&str> for Command {
    fn from(string: &str) -> Self {
        match string {
            "!back-merge" | "!bm" => Command::BackMerge,
            "!release" | "!r" => Command::Release,
            "!update-release" | "!ur" => Command::UpdateRelease,
            "!merge" | "!m" => Command::Merge,
            "!ping" | "!p" => Command::Ping,
            _ => Command::Noop,
        }
    }
}

impl Command {
    pub fn execute(
        &self,
        github_client: GithubClient,
        issue_comment_payload: IssueCommentEventPayload,
    ) {
        match self {
            Command::BackMerge => {
                BackMerge::execute(github_client, issue_comment_payload.repository_full_name())
            }
            Command::Release => Release::execute(github_client, issue_comment_payload),
            Command::UpdateRelease => UpdateRelease::execute(github_client, issue_comment_payload),
            Command::Ping => Ping::execute(github_client, issue_comment_payload),
            Command::Merge => Merge::execute(github_client, issue_comment_payload),
            Command::Noop => {}
        }
    }
}
