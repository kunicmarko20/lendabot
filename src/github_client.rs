use reqwest::{Response, Client, header::{AUTHORIZATION, HeaderValue, HeaderMap}};
use std::env;

type Result = reqwest::Result<Response>;

lazy_static! {
    static ref TOKEN: String = env::var("APP_GITHUB_API_TOKEN").expect("APP_GITHUB_API_TOKEN not set.");
}

pub(crate) struct GithubClient {
    client: Client,
}

impl GithubClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_static(&TOKEN));

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
}
