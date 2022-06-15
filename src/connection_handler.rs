use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use serde::{Serialize, Deserialize};
use serde_json;


#[derive(Debug, Serialize, Deserialize)]
struct Subscribe {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum ClientMessage {
    Subscribe(Subscribe),
}

fn transform_u32_to_array_of_u8(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

fn send_message(mut stream: TcpStream, message: &str) {
    let message_size: u32 = message.len() as u32;
    let encoded_size = &transform_u32_to_array_of_u8(message_size);

    let response = stream.write(encoded_size);
    match response {
        Err(error) => panic!("Error writing to server: {error}"),
        _ => {}
    }

    let response = stream.write(message.as_bytes());
    match response {
        Err(error) => panic!("Error writing to server: {error}"),
        _ => {}
    }
}

pub(crate) fn connect() {
    let stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            say_hello(stream);
        }
        Err(err) => panic!("Cannot connect : {err}"),
    }
}

fn say_hello(mut stream: TcpStream) {
    send_message(stream, "\"Hello\"");
}

fn listen_from_stream(mut stream: TcpStream) {
    let listener = TcpListener::bind(stream.local_addr().unwrap());
    let listener = match listener {
        Ok(l) => l,
        Err(err) => panic!("Cannot listen on port : {err:?}")
    };

    for message in listener.incoming() {
        println!("{:?}", message);
    }
}
