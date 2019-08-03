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
    let slash_command_payload: SlashCommandPayload =
        serde_urlencoded::from_bytes(request.body().as_ref()).unwrap();

    let command: Command = slash_command_payload.command().into();
    command.execute(GithubClient::default(), slash_command_payload);

    Ok(Response::builder()
        .status(200)
        .body("".into())
        .expect("Error while creating response."))
}
