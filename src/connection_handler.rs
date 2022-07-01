use std::io::{Read, Write};
use std::net::{TcpStream};
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json;
use crate::challenge::Challenge;
use crate::challenge_message::{ChallengeOutput, MD5HashCashInput, MD5HashCashOutput, RecoverSecretInput, RecoverSecretOutput};
use crate::challenge_message::Challenge::{MD5HashCash, RecoverSecret};

use crate::client_message::{ClientMessage, Subscribe};
use crate::md5cash_challenge::HashCash;
use crate::recover_secret_challenge::{Recover};
use crate::server_message::ServerMessage;

pub(crate) fn connect(username: String, port: u16) {
    let stream = TcpStream::connect("localhost:{port}".replace("{port}", &port.to_string()));
    match stream {
        Ok(stream) => {
            say_hello(&stream);
            listen_from_stream(&stream, username);
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

    println!("Sending message \"{message}\" of length {message_size}", message=message, message_size=message_size);

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

fn listen_from_stream(stream: &TcpStream, username: String) {
    let mut is_connection_opened = true;

    while is_connection_opened {
        let message = read_message(&stream);
        println!("Message: {:?}", message);

        let message_json = serde_json::from_str(&message).unwrap();
        match message_json {
            ServerMessage::Welcome(welcome) => {
                println!("Welcome: {:?}", welcome);
                send_username(&stream, &username);
            }
            ServerMessage::SubscribeResult(subscribe_result) => {
                println!("Subscribe result: {:?}", subscribe_result);
            }
            ServerMessage::PublicLeaderBoard(public_leader_board) => {
                println!("PublicLeaderBoard: {:?}", public_leader_board);
            }
            ServerMessage::RoundSummary(round_summary) => {
                println!("RoundSummary: {:?}", round_summary);
            }
            ServerMessage::Challenge(challenge) => {
                println!("Challenge: {:?}", challenge);
                match challenge {
                    MD5HashCash(md5_hash_cash) => {
                        println!("MD5HashCash: {:?}", md5_hash_cash);
                        let mut hashcash = HashCash::new(md5_hash_cash);
                        let mut hashcash_result = &HashCash::solve(&hashcash);
                        let hashcash_output = ChallengeOutput::MD5HashCash(MD5HashCashOutput {
                            seed: hashcash_result.seed,
                            hashcode: hashcash_result.hashcode.to_string(),
                        });
                        send_message(&stream, &serde_json::to_string(&hashcash_output).unwrap());
                    }
                    RecoverSecret(recover_secret) => {
                        println!("RecoverSecret: {:?}", recover_secret);
                        let mut recover_secret = Recover::new(recover_secret);
                        let mut recover_secret_result = &Recover::solve(&recover_secret);
                        let recover_secret_output = ChallengeOutput::RecoverSecret(RecoverSecretOutput {
                            secret_sentence: recover_secret_result.secret_sentence.to_string()
                        });
                        send_message(&stream, &serde_json::to_string(&recover_secret_output).unwrap());
                    }
                }
            }
            ServerMessage::EndOfGame(end_of_game) => {
                println!("EndOfGame: {:?}", end_of_game);
                is_connection_opened = false;
                stream.shutdown(std::net::Shutdown::Both).unwrap();
            }
        }
    }
}
