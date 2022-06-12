use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Hello {
}

#[derive(Debug, Serialize, Deserialize)]
struct Subscribe {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum ClientMessage {
    Hello(Hello),
    Subscribe(Subscribe),
}

pub(crate) fn connect() {
    let stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            // say_hello(stream);
            let message= "Hi!".as_bytes();
            let response = stream.write(&message);
            println!("response={:?}", response);
            println!("reading={:?}", stream.read(&mut [0; 128]).unwrap());

            let subscribe_message = ClientMessage::Subscribe(Subscribe {
                name: "test".to_string(),
            });

            let serialized = serde_json::to_string(&subscribe_message);

            match serialized {
                Ok(str) => {
                    println!("ok:{str}");
                    let message= str.as_bytes();
                    let response = stream.write(&message);
                    println!("{:?}", response);
                }
                Err(err) => {
                    println!("{err}")
                }
            }
        }
        Err(err) => panic!("Cannot connect : {err}"),
    }
}

fn say_hello(mut stream: TcpStream) {
    let hello = ClientMessage::Hello(Hello {});
    let message = serde_json::to_string(&hello).unwrap();
    let response = stream.write_all(message.as_bytes());
    println!("{:?}", response);
    let mut buffer = [0; 1024];
    let response = stream.read(&mut buffer);
    println!("{:?}", response);
    let str = String::from_utf8_lossy(&buffer);
    println!("{}", str);
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
