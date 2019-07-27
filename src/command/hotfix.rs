use super::GITHUB_CLIENT;

pub struct Hotfix;

impl Hotfix {
    pub fn execute(repository_full_name: &String) {
        let body = json!({
            "title": "🤖 Hotfix back-merge",
            "head": "master",
            "base": "development",
            "maintainer_can_modify": true,
            "body": "Beep boop, your wish is my command."
        });

        GITHUB_CLIENT.create_pull_request(
            repository_full_name,
            body.to_string()
        ).unwrap();
    }
}
