#[macro_use] extern crate lambda_runtime as lambda;

use std::error::Error;
use lambda_runtime::error::HandlerError;
use serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(run);
    Ok(())
}

pub fn run(event: Value, context: lambda::Context) -> Result<(), HandlerError> {
    //Your code here
    Ok(())
}
