use lendabot::Command;
use std::env;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use lendabot::slack::payload::SlashCommandPayload;

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

    let slash_command_payload: SlashCommandPayload =
        serde_urlencoded::from_bytes(request.trim().as_ref()).unwrap();

    let command: Command = slash_command_payload.command().as_str().into();
    command.execute(&slash_command_payload);

    ok(stream)
}

fn request(mut stream: &TcpStream) -> String {
    let mut buffer = [0; 65536];

    let number_of_characters = stream.read(&mut buffer).unwrap();

    let raw_body = String::from_utf8_lossy(&buffer[0..number_of_characters]);

    raw_body.lines().skip(10).collect()
}

fn ok(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
