use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::env;
use lendabot::{Payload, Command};
use std::borrow::Cow;

fn main() {
    let address = env::var("APP_ADDRESS").expect("APP_ADDRESS not set.");
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 16384];

    let number_of_characters = stream.read(&mut buffer).unwrap();

    let raw_body = String::from_utf8_lossy(&buffer[0..number_of_characters]);
    let body = get_request_body(raw_body);

    let payload: Payload = serde_json::from_str(body.trim()).unwrap();

    if payload.is_pull_request() {
        let command: Command = payload.comment_body().as_str().into();
        command.execute(&payload);
    }

    ok(stream)
}

fn get_request_body(raw_body: Cow<str>) -> String {
    raw_body.lines().skip(10).collect()
}

fn ok(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
