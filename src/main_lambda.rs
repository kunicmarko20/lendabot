#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use std::error::Error;
use lambda_runtime::{error::HandlerError, Context};
use lambda_http::{lambda, Request, RequestExt, Response, Body};
use reqwest::header::AUTHORIZATION;
use regex::Regex;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(router);
    Ok(())
}

#[derive(Deserialize, Debug)]
struct Payload {
    comment: Comment,
    repository: Repository,
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"https://github.com/(\w+)/(\w+)/pull/(\d+)").unwrap();
}

impl Payload {
    pub fn is_pull_request(&self) -> bool {
        REGEX.is_match(&self.comment.html_url)
    }
}

#[derive(Deserialize, Debug)]
struct Comment {
    html_url: String,
    body: String,
}

#[derive(Deserialize, Debug)]
struct Repository {
    full_name: String,
}

pub fn router(request: Request, _: Context) -> Result<Response<Body>, HandlerError> {
    let payload = request.payload::<Payload>().unwrap().unwrap();

    if payload.is_pull_request() {
        let command: Command = payload.comment.body.as_str().into();
        command.execute(&payload);
    }

    Ok(Response::builder()
        .status(200)
        .body("".into())
        .expect("Error while creating response."))
}

enum Command {
    Hotfix,
    Release,
    Noop,
}

impl From<&str> for Command {
    fn from(string: &str) -> Self {
        match string {
            "!hotfix" => Command::Hotfix,
            "!release" => Command::Release,
            _ => Command::Noop,
        }
    }
}

impl Command {
    pub fn execute(&self, payload: &Payload) {
        match self {
            Command::Hotfix => self.execute_hotfix(&payload),
            Command::Release => self.execute_release(&payload),
            Command::Noop => {},
        }
    }

    fn execute_hotfix(&self, payload: &Payload) {
        let token = env::var("APP_GITHUB_API_TOKEN").expect("APP_GITHUB_API_TOKEN not set.");

        let request_payload = json!({
            "title": "🤖 Hotfix back-merge into development",
            "head": "master",
            "base": "development",
            "maintainer_can_modify": true,
            "body": "Beep boop, your request is my command."
        });

        let mut response = reqwest::Client::new()
            .post(&format!("https://api.github.com/repos/{}/pulls", payload.repository.full_name))
            .header(AUTHORIZATION, format!("token {}", token))
            .body(request_payload.to_string())
            .send().unwrap();

        println!("{}", response.text().expect("Unable to grab response content."));
    }

    fn execute_release(&self, payload: &Payload) {
        let token = env::var("APP_GITHUB_API_TOKEN").expect("APP_GITHUB_API_TOKEN not set.");

        let request_payload = json!({
            "title": "🤖 Release",
            "head": "development",
            "base": "master",
            "maintainer_can_modify": true,
            "body": "Beep boop, your request is my command."
        });

        let mut response = reqwest::Client::new()
            .post(&format!("https://api.github.com/repos/{}/pulls", payload.repository.full_name))
            .header(AUTHORIZATION, format!("token {}", token))
            .body(request_payload.to_string())
            .send().unwrap();

        println!("{}", response.text().expect("Unable to grab response content."));
    }
}

#[cfg(test)]
mod tests {
    use crate::REGEX;

    #[test]
    fn it_will_match_regex() {
        assert!(REGEX.is_match("https://github.com/i_am_an_owner/this_is_repository/pull/3"));
        assert!(REGEX.is_match("https://github.com/aws/das/pull/3"));
    }

    #[test]
    fn it_will_not_match_regex() {
        assert!(!REGEX.is_match("https://github.com/i_am_an_owner/this_is_repository/issues/3"));
        assert!(!REGEX.is_match("https://github.com/aws/das/issues/3"));
    }
}
