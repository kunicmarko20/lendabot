use std::error::Error;
use lambda_runtime::{error::HandlerError, Context};
use lambda_http::{lambda, Request, Response, Body};

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(run);
    Ok(())
}

pub fn run(request: Request, _: Context) -> Result<Response<Body>, HandlerError> {
    println!("{}", std::str::from_utf8(request.body().as_ref()).unwrap());

    Ok(Response::builder()
        .status(200)
        .body("Ok".into())
        .expect("Error while creating response."))
}
