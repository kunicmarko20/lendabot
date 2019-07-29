mod back_merge;
mod merge;
mod ping;
mod release;
mod update_release;

pub use self::back_merge::BackMerge;

use self::merge::Merge;
use self::ping::Ping;
use self::release::Release;
use self::update_release::UpdateRelease;
use crate::github::payload::IssueCommentEventPayload;
use crate::github::GithubClient;

lazy_static! {
    static ref GITHUB_CLIENT: GithubClient = GithubClient::default();
}

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
    pub fn execute(&self, issue_comment_payload: &IssueCommentEventPayload) {
        match self {
            Command::BackMerge => BackMerge::execute(&issue_comment_payload.repository_full_name()),
            Command::Release => Release::execute(&issue_comment_payload.repository_full_name()),
            Command::UpdateRelease => UpdateRelease::execute(
                &issue_comment_payload.repository_full_name(),
                *issue_comment_payload.issue_number(),
            ),
            Command::Ping => Ping::execute(
                &issue_comment_payload.repository_full_name(),
                *issue_comment_payload.issue_number(),
            ),
            Command::Merge => Merge::execute(
                &issue_comment_payload.repository_full_name(),
                &issue_comment_payload.issue_title(),
                *issue_comment_payload.issue_number(),
            ),
            Command::Noop => {}
        }
    }
}
