use std::error::Error;
use lambda_runtime::{error::HandlerError, Context};
use lambda_http::{lambda, Request, RequestExt, Response, Body};
use lendabot::{Command, Payload};

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(run);
    Ok(())
}

pub fn run(request: Request, _: Context) -> Result<Response<Body>, HandlerError> {
    let payload = request.payload::<Payload>().unwrap().unwrap();

    if payload.is_pull_request() {
        let command: Command = payload.comment_body().as_str().into();
        command.execute(&payload);
    }

    Ok(Response::builder()
        .status(200)
        .body("".into())
        .expect("Error while creating response."))
}
