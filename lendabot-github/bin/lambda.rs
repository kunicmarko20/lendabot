use lambda_http::{lambda, Body, Request, RequestExt, Response};
use lambda_runtime::{error::HandlerError, Context};
use lendabot_github::command::BackMerge;
use lendabot_github::payload::{IssueCommentEventPayload, PullRequestEventPayload};
use lendabot_github::Command;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(run);
    Ok(())
}

pub fn run(request: Request, _: Context) -> Result<Response<Body>, HandlerError> {
    let event: Event = request
        .headers()
        .get("X-GitHub-Event")
        .unwrap()
        .to_str()
        .unwrap()
        .into();

    event.execute(&request);

    Ok(Response::builder()
        .status(200)
        .body("".into())
        .expect("Error while creating response."))
}

#[derive(Debug)]
enum Event {
    IssueComment,
    PullRequest,
    Noop,
}

impl From<&str> for Event {
    fn from(string: &str) -> Self {
        match string {
            "issue_comment" => Event::IssueComment,
            "pull_request" => Event::PullRequest,
            _ => Event::Noop,
        }
    }
}

impl Event {
    pub fn execute(&self, request: &Request) {
        match self {
            Event::IssueComment => self.execute_issue_comment(request),
            Event::PullRequest => self.execute_pull_request(request),
            Event::Noop => {}
        }
    }

    fn execute_issue_comment(&self, request: &Request) {
        let issue_comment_payload = request
            .payload::<IssueCommentEventPayload>()
            .unwrap()
            .unwrap();

        if issue_comment_payload.is_pull_request() {
            let command: Command = issue_comment_payload.comment_body().into();
            command.execute(issue_comment_payload);
        }
    }

    fn execute_pull_request(&self, request: &Request) {
        let pull_request_payload = request
            .payload::<PullRequestEventPayload>()
            .unwrap()
            .unwrap();

        if pull_request_payload.is_merged() && pull_request_payload.is_hotfix() {
            BackMerge::execute(pull_request_payload.repository_full_name());
        }
    }
}
