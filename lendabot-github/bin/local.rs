use lendabot_github::command::BackMerge;
use lendabot_github::payload::{IssueCommentEventPayload, PullRequestEventPayload};
use lendabot_github::Command;
use std::env;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let address = env::var("APP_ADDRESS").expect("APP_ADDRESS not set.");
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    let request = request(&stream);

    request.0.execute(request.1);

    ok(stream)
}

fn request(mut stream: &TcpStream) -> (Event, String) {
    let mut buffer = [0; 65536];

    let number_of_characters = stream.read(&mut buffer).unwrap();

    let raw_body = String::from_utf8_lossy(&buffer[0..number_of_characters]);

    (
        raw_body.lines().nth(4).unwrap().trim().into(),
        raw_body.lines().skip(9).collect(),
    )
}

fn ok(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
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
            "X-GitHub-Event: issue_comment" => Event::IssueComment,
            "X-GitHub-Event: pull_request" => Event::PullRequest,
            _ => Event::Noop,
        }
    }
}

impl Event {
    pub fn execute(&self, request: String) {
        match self {
            Event::IssueComment => self.execute_issue_comment(request),
            Event::PullRequest => self.execute_pull_request(request),
            Event::Noop => {}
        }
    }

    fn execute_issue_comment(&self, request: String) {
        let issue_comment_payload: IssueCommentEventPayload =
            serde_json::from_str(request.trim()).unwrap();

        if issue_comment_payload.is_pull_request() {
            let command: Command = issue_comment_payload.comment_body().into();
            command.execute(issue_comment_payload);
        }
    }

    fn execute_pull_request(&self, request: String) {
        let pull_request_payload: PullRequestEventPayload =
            serde_json::from_str(request.trim()).unwrap();

        if pull_request_payload.is_merged() && pull_request_payload.is_hotfix() {
            BackMerge::execute(pull_request_payload.repository_full_name());
        }
    }
}
