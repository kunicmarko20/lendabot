use lambda_http::{lambda, Body, Request, RequestExt, Response};
use lambda_runtime::{error::HandlerError, Context};
use lendabot::Command;
use std::error::Error;
use lendabot::slack::payload::SlashCommandPayload;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(run);
    Ok(())
}

pub fn run(request: Request, _: Context) -> Result<Response<Body>, HandlerError> {
    let slash_command_payload: SlashCommandPayload =
        serde_urlencoded::from_bytes(request.body().as_ref()).unwrap();

    let command: Command = slash_command_payload.command().as_str().into();
    command.execute(&slash_command_payload);

    Ok(Response::builder()
        .status(200)
        .body("".into())
        .expect("Error while creating response."))
}
