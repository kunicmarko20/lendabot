use super::GITHUB_CLIENT;

pub(super) struct Ping;

impl Ping {
    pub fn execute(repository_full_name: &str, pull_request_number: u64) {
        let body = json!({
            "body": "pong"
        });

        GITHUB_CLIENT
            .create_comment(repository_full_name, pull_request_number, body.to_string())
            .unwrap();
    }
}
