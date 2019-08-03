use lendabot::github::GithubClient;

pub struct BackMerge;

impl BackMerge {
    pub fn execute(github_client: GithubClient, repository_full_name: &str) {
        let body = json!({
            "title": "🤖 Back-merge",
            "head": "master",
            "base": "development",
            "maintainer_can_modify": true,
        });

        github_client.create_pull_request(repository_full_name, body.to_string());
    }
}
