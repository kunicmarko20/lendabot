use reqwest::{Response, Client, header::{AUTHORIZATION, HeaderValue, HeaderMap}};
use std::env;

type Result = reqwest::Result<Response>;

lazy_static! {
    static ref TOKEN: String = env::var("APP_GITHUB_API_TOKEN").expect("APP_GITHUB_API_TOKEN not set.");
    static ref AUTHORIZATION_HEADER_VALUE: String = "token ".to_string() + &TOKEN;
}

pub(crate) struct GithubClient {
    client: Client,
}

impl GithubClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_static(&AUTHORIZATION_HEADER_VALUE));

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        GithubClient{client}
    }

    pub fn create_pull_request(&self, repository_name: &String, body: String) -> Result {
        self.client
            .post(&format!("https://api.github.com/repos/{}/pulls", repository_name))
            .body(body)
            .send()
    }

    pub fn create_comment(&self, repository_name: &String, issue_number: &u64, body: String) -> Result {
        self.client
            .post(&format!("https://api.github.com/repos/{}/issues/{}/comments", repository_name, issue_number))
            .body(body)
            .send()
    }
}
