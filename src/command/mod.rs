mod hotfix;
mod merge;
mod ping;
mod release;

use crate::github::{IssueComment, GithubClient};
use self::hotfix::Hotfix;
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
    pub fn execute(&self, issue_comment: &IssueComment) {
        match self {
            Command::Hotfix => Hotfix::execute(&issue_comment),
            Command::Release => Release::execute(&issue_comment),
            Command::Ping => Ping::execute(&issue_comment),
            Command::Merge => Merge::execute(&issue_comment),
            Command::Noop => {},
        }
    }
}
