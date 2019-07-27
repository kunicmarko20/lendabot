use std::error::Error;
use lambda_runtime::{error::HandlerError, Context};
use lambda_http::{lambda, Request, RequestExt, Response, Body};
use lendabot::Command;
use lendabot::github::IssueComment;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(run);
    Ok(())
}

pub fn run(request: Request, _: Context) -> Result<Response<Body>, HandlerError> {
    let issue_comment = request.payload::<IssueComment>().unwrap().unwrap();

    if issue_comment.is_pull_request() {
        let command: Command = issue_comment.comment_body().as_str().into();
        command.execute(&issue_comment);
    }

    Ok(Response::builder()
        .status(200)
        .body("".into())
        .expect("Error while creating response."))
}
