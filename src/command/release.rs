use super::UpdateRelease;
use super::GITHUB_CLIENT;

pub(super) struct Release;

impl Release {
    pub fn execute(repository_full_name: &str) {
        let body = json!({
            "title": "🤖 Release",
            "head": "development",
            "base": "master",
            "maintainer_can_modify": true,
        });

        let pull_request =
            GITHUB_CLIENT.create_pull_request(repository_full_name, body.to_string());

        UpdateRelease::execute(repository_full_name, *pull_request.pull_request_number());
    }
}
