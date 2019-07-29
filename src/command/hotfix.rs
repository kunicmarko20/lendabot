use super::GITHUB_CLIENT;

pub struct Hotfix;

impl Hotfix {
    pub fn execute(repository_full_name: &str) {
        let body = json!({
            "title": "🤖 Hotfix back-merge",
            "head": "master",
            "base": "development",
            "maintainer_can_modify": true,
        });

        GITHUB_CLIENT.create_pull_request(repository_full_name, body.to_string());
    }
}
