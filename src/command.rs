use crate::Payload;
use crate::github_client::GithubClient;

lazy_static! {
    static ref GITHUB_CLIENT: GithubClient = GithubClient::new();
}

pub enum Command {
    Hotfix,
    Release,
    Ping,
    Noop,
}

impl From<&str> for Command {
    fn from(string: &str) -> Self {
        match string {
            "!hotfix" => Command::Hotfix,
            "!release" => Command::Release,
            "!ping" => Command::Ping,
            _ => Command::Noop,
        }
    }
}

impl Command {
    pub fn execute(&self, payload: &Payload) {
        match self {
            Command::Hotfix => self.execute_hotfix(&payload),
            Command::Release => self.execute_release(&payload),
            Command::Ping => self.execute_ping(&payload),
            Command::Noop => {},
        }
    }

    fn execute_hotfix(&self, payload: &Payload) {
        let body = json!({
            "title": "🤖 Hotfix back-merge",
            "head": "master",
            "base": "development",
            "maintainer_can_modify": true,
            "body": "Beep boop, your wish is my command."
        });

        GITHUB_CLIENT.create_pull_request(
            payload.repository_full_name(),
            body.to_string()
        ).unwrap();
    }

    fn execute_release(&self, payload: &Payload) {
        let body = json!({
            "title": "🤖 Release",
            "head": "development",
            "base": "master",
            "maintainer_can_modify": true,
            "body": "Beep boop, your wish is my command."
        });

        GITHUB_CLIENT.create_pull_request(
            payload.repository_full_name(),
            body.to_string()
        ).unwrap();
    }

    fn execute_ping(&self, payload: &Payload) {
        let body = json!({
            "body": "pong"
        });

        GITHUB_CLIENT.create_comment(
            payload.repository_full_name(),
            payload.issue_number(),
            body.to_string()
        ).unwrap();
    }
}
