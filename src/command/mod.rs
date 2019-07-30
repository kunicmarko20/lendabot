#[cfg(not(any(feature = "lambda-github", feature = "lambda-slack")))]
compile_error!("Either feature \"lambda-github\" or \"lambda-slack\" must be enabled for this crate.");

cfg_if! {
    if #[cfg(feature = "lambda-github")] {
        mod back_merge;
        mod command_github;
        mod merge;
        mod ping;

        pub use self::back_merge::BackMerge;
        pub use command_github::Command;

        pub(in crate::command) use self::merge::Merge;
        pub(in crate::command) use self::ping::Ping;
    }
}

cfg_if! {
    if #[cfg(feature = "lambda-slack")] {
        mod command_slack;
        pub use command_slack::Command;
    }
}

mod release;
mod update_release;

pub(in crate::command) use self::release::Release;
pub(in crate::command) use self::update_release::UpdateRelease;
use crate::github::GithubClient;

lazy_static! {
    static ref GITHUB_CLIENT: GithubClient = GithubClient::default();
}
