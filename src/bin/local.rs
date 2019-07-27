use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::env;
use lendabot::{Payload, Command};

fn main() {
    let address = env::var("APP_ADDRESS").expect("APP_ADDRESS not set.");
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    let body = get_request_body(&stream);
    let payload: Payload = serde_json::from_str(body.trim()).unwrap();

    if payload.is_pull_request() {
        let command: Command = payload.comment_body().as_str().into();
        command.execute(&payload);
    }

    ok(stream)
}

fn get_request_body(mut stream: &TcpStream) -> String {
    let mut buffer = [0; 65536];

    let number_of_characters = stream.read(&mut buffer).unwrap();

    let raw_body = String::from_utf8_lossy(&buffer[0..number_of_characters]);

    raw_body.lines().skip(10).collect()
}

fn ok(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
