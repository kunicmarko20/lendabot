use super::GITHUB_CLIENT;

pub struct BackMerge;

impl BackMerge {
    pub fn execute(repository_full_name: &str) {
        let body = json!({
            "title": "🤖 Back-merge",
            "head": "master",
            "base": "development",
            "maintainer_can_modify": true,
        });

        GITHUB_CLIENT.create_pull_request(repository_full_name, body.to_string());
    }
}
