use lambda_http::{lambda, Body, Request, Response};
use lambda_runtime::{error::HandlerError, Context};
use lendabot::github::GithubClient;
use lendabot_slack::payload::SlashCommandPayload;
use lendabot_slack::Command;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(run);
    Ok(())
}

pub fn run(request: Request, _: Context) -> Result<Response<Body>, HandlerError> {
    let slash_command_payload = SlashCommandPayload::from_request_body(
        serde_urlencoded::from_bytes(request.body().as_ref()).unwrap(),
    );

    if !slash_command_payload.has_permissions() {
        return Ok(Response::builder()
            .status(200)
            .body("You don't have permission to execute that.".into())
            .expect("Error while creating response."));
    }

    let command: Command = slash_command_payload.command().into();
    let response = command.execute(GithubClient::default(), slash_command_payload);

    if let Err(message) = response {
        return Ok(Response::builder()
            .status(200)
            .body(message.into())
            .expect("Error while creating response."));
    }

    Ok(Response::builder()
        .status(200)
        .body("".into())
        .expect("Error while creating response."))
}
