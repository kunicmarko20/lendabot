mod hotfix;
mod merge;
mod ping;
mod release;
mod update_release;

pub use self::hotfix::Hotfix;

use crate::github::GithubClient;
use crate::github::payload::IssueCommentEventPayload;
use self::merge::Merge;
use self::ping::Ping;
use self::release::Release;
use self::update_release::UpdateRelease;

lazy_static! {
    static ref GITHUB_CLIENT: GithubClient = GithubClient::new();
}

#[derive(Debug)]
pub enum Command {
    Hotfix,
    Release,
    UpdateRelease,
    Ping,
    Merge,
    Noop,
}

impl From<&str> for Command {
    fn from(string: &str) -> Self {
        match string {
            "!hotfix" | "!h" => Command::Hotfix,
            "!release" | "!r" => Command::Release,
            "!urelease" | "!ur" => Command::UpdateRelease,
            "!merge" | "!m" => Command::Merge,
            "!ping" | "!p" => Command::Ping,
            _ => Command::Noop,
        }
    }
}

impl Command {
    pub fn execute(&self, issue_comment_payload: &IssueCommentEventPayload) {
        match self {
            Command::Hotfix => Hotfix::execute(&issue_comment_payload.repository_full_name()),
            Command::Release => Release::execute(&issue_comment_payload),
            Command::UpdateRelease => UpdateRelease::execute(
                &issue_comment_payload.repository_full_name(),
                &issue_comment_payload.issue_number()
            ),
            Command::Ping => Ping::execute(&issue_comment_payload),
            Command::Merge => Merge::execute(&issue_comment_payload),
            Command::Noop => {},
        }
    }
}
