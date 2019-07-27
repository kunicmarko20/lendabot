mod hotfix;
mod merge;
mod ping;
mod release;

pub use self::hotfix::Hotfix;

use crate::github::GithubClient;
use crate::github::payload::IssueCommentEventPayload;
use self::merge::Merge;
use self::ping::Ping;
use self::release::Release;

lazy_static! {
    static ref GITHUB_CLIENT: GithubClient = GithubClient::new();
}

#[derive(Debug)]
pub enum Command {
    Hotfix,
    Release,
    Ping,
    Merge,
    Noop,
}

impl From<&str> for Command {
    fn from(string: &str) -> Self {
        match string {
            "!hotfix" => Command::Hotfix,
            "!release" => Command::Release,
            "!merge" => Command::Merge,
            "!ping" => Command::Ping,
            _ => Command::Noop,
        }
    }
}

impl Command {
    pub fn execute(&self, issue_comment_payload: &IssueCommentEventPayload) {
        match self {
            Command::Hotfix => Hotfix::execute(&issue_comment_payload.repository_full_name()),
            Command::Release => Release::execute(&issue_comment_payload),
            Command::Ping => Ping::execute(&issue_comment_payload),
            Command::Merge => Merge::execute(&issue_comment_payload),
            Command::Noop => {},
        }
    }
}
