use super::payload::PullRequestCommitPayload;
use super::payload::PullRequestPayload;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, Response,
};
use std::env;
use crate::github::payload::ErrorPayload;

type ResultReqwest = reqwest::Result<Response>;

lazy_static! {
    static ref TOKEN: String =
        env::var("APP_GITHUB_API_TOKEN").expect("APP_GITHUB_API_TOKEN not set.");
    static ref AUTHORIZATION_HEADER_VALUE: String = "token ".to_string() + &TOKEN;
}

pub struct GithubClient {
    client: Client,
}

impl Default for GithubClient {
    fn default() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_static(&AUTHORIZATION_HEADER_VALUE),
        );

        let client = Client::builder().default_headers(headers).build().unwrap();

        GithubClient { client }
    }
}

impl GithubClient {
    pub fn create_pull_request(&self, repository_name: &str, body: String) -> Result<PullRequestPayload, ErrorPayload> {
        let mut response = self.client
            .post(&format!(
                "https://api.github.com/repos/{}/pulls",
                repository_name
            ))
            .body(body)
            .send()
            .unwrap();

        if !response.status().is_success() {
            return Err(response
                .json()
                .unwrap());
        }

        Ok(response
            .json()
            .unwrap())
    }

    pub fn update_pull_request(
        &self,
        repository_name: &str,
        pull_request_number: u64,
        body: String,
    ) -> ResultReqwest {
        self.client
            .patch(&format!(
                "https://api.github.com/repos/{}/pulls/{}",
                repository_name, pull_request_number
            ))
            .body(body)
            .send()
    }

    pub fn create_comment(&self, repository_name: &str, issue_number: u64, body: String) -> ResultReqwest {
        self.client
            .post(&format!(
                "https://api.github.com/repos/{}/issues/{}/comments",
                repository_name, issue_number
            ))
            .body(body)
            .send()
    }

    pub fn merge_pull_request(
        &self,
        repository_name: &str,
        pull_request_number: u64,
        body: String,
    ) -> ResultReqwest {
        self.client
            .put(&format!(
                "https://api.github.com/repos/{}/pulls/{}/merge",
                repository_name, pull_request_number
            ))
            .body(body)
            .send()
    }

    pub fn pull_request_info(
        &self,
        repository_name: &str,
        pull_request_number: u64,
    ) -> PullRequestPayload {
        self.client
            .get(&format!(
                "https://api.github.com/repos/{}/pulls/{}",
                repository_name, pull_request_number
            ))
            .send()
            .unwrap()
            .json()
            .unwrap()
    }

    pub fn pull_request_commits(
        &self,
        repository_name: &str,
        pull_request_number: u64,
    ) -> Vec<PullRequestCommitPayload> {
        self.client
            .get(&format!(
                "https://api.github.com/repos/{}/pulls/{}/commits",
                repository_name, pull_request_number
            ))
            .send()
            .unwrap()
            .json()
            .unwrap()
    }
}
