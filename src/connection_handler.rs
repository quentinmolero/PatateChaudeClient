use std::io::{Read, Write};
use std::net::{TcpStream};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
enum ChallengeName {
    HashCash,
    RecoverSecret,
}

#[derive(Debug, Serialize, Deserialize)]
struct Subscribe {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum ChallengeOutput {
    HashCash(String),
    RecoverSecret(String),
}

#[derive(Debug, Serialize, Deserialize)]
enum ChallengeAnswer {
    ChallengeName(ChallengeOutput),
}

#[derive(Debug, Serialize, Deserialize)]
struct ChallengeResult {
    name: ChallengeAnswer,
    next_target: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum ClientMessage {
    Subscribe(Subscribe),
}

#[derive(Debug, Serialize, Deserialize)]
struct Welcome {
    version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
enum SubscriptionError {
    AlreadyRegistered,
    InvalidName
}

#[derive(Debug, Serialize, Deserialize)]
struct SubscribeError {
    subscription_error: SubscriptionError,
}

#[derive(Debug, Serialize, Deserialize)]
enum Result {
    Ok,
    SubscribeError(SubscribeError),
}

#[derive(Debug, Serialize, Deserialize)]
struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct SubscribeResult {
    result: Result,
}

#[derive(Debug, Serialize, Deserialize)]
struct Challenge {
    challenge: String,
    chain: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
enum ServerMessage {
    Welcome(Welcome),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
}

pub(crate) fn connect() {
    let stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(stream) => {
            say_hello(&stream);
            println!("{:?}", read_message(&stream));
            send_username(&stream, ("Player".to_string() + &SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string()).as_str());
            println!("{:?}", read_message(&stream));
            listen_from_stream(&stream);
        }
        Err(err) => panic!("Cannot connect : {err}"),
    }
}

fn transform_u32_to_array_of_u8(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

fn transform_array_of_u8_to_u32(x:[u8;4]) -> u32 {
    ((x[0] as u32) << 24) | ((x[1] as u32) << 16) | ((x[2] as u32) << 8) | (x[3] as u32)
}

fn send_message(mut stream: &TcpStream, message: &str) {
    let message_size: u32 = message.len() as u32;
    let encoded_size = &transform_u32_to_array_of_u8(message_size);

    // println!("Sending message \"{message}\" of length {message_size}", message=message, message_size=message_size);

    let response = stream.write(encoded_size);
    match response {
        Err(error) => {
            panic!("{:?}", error);
        }
        _ => {}
    }

    let response = stream.write(message.as_bytes());
    match response {
        Err(error) => {
            panic!("{:?}", error);
        }
        _ => {}
    }
}

fn read_message(mut stream: &TcpStream) -> String {
    let mut length_buffer = [0; 4];
    let buffer_response = stream.read(&mut length_buffer);
    match buffer_response {
        Err(error) => {
            panic!("{:?}", error)
        }
        _ => {}
    }

    let length = transform_array_of_u8_to_u32(length_buffer);
    let mut buffer = vec![0; length as usize];
    let response = stream.read(&mut buffer);
    match response {
        Err(error) => {
            panic!("{:?}", error);
        }
        _ => {}
    }

    let message = String::from_utf8_lossy(&buffer);
    // println!("Received message \"{message}\" of length {message_size}", message = message, message_size = length);
    return message.to_string();
}

fn say_hello(stream: &TcpStream) {
    send_message(stream, "\"Hello\"");
}

fn send_username(stream: &TcpStream, username: &str) {
    let message = ClientMessage::Subscribe(Subscribe {
        name: username.to_string(),
    });
    let message_json = serde_json::to_string(&message).unwrap();
    send_message(stream, &message_json);
}

fn listen_from_stream(stream: &TcpStream) {
    loop {
        let message = read_message(&stream);
        println!("{:?}", message);

        let message_json = serde_json::from_str(&message).unwrap();
        match message_json {
            ServerMessage::PublicLeaderBoard(public_leader_board) => {
                println!("PublicLeaderBoard: {:?}", public_leader_board);
            }
            ServerMessage::Challenge(challenge) => {
                println!("Challenge: {:?}", challenge);
            }
            _ => {}
        }
    }
}
