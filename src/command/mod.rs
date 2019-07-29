mod back_merge;
#[cfg(feature = "lambda-github")]
mod command_github;
mod merge;
mod ping;
mod release;
mod update_release;

pub use self::back_merge::BackMerge;

pub(in crate::command) use self::merge::Merge;
pub(in crate::command) use self::ping::Ping;
pub(in crate::command) use self::release::Release;
pub(in crate::command) use self::update_release::UpdateRelease;
use crate::github::GithubClient;

lazy_static! {
    static ref GITHUB_CLIENT: GithubClient = GithubClient::default();
}

#[cfg(feature = "lambda-github")]
pub use command_github::Command;
