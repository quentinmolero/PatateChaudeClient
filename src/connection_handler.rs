use std::io::{Read, Write};
use std::net::{TcpStream};
use serde_json;
use crate::challenge::Challenge;
use crate::challenge_message::{ChallengeOutput, ChallengeResult, MD5HashCashOutput, RecoverSecretOutput};
use crate::challenge_message::Challenge::{MD5HashCash, RecoverSecret};

use crate::client_message::{ClientMessage, Subscribe};
use crate::md5cash_challenge::HashCash;
use crate::recover_secret_challenge::{Recover};
use crate::server_message::{PublicPlayer, ServerMessage};
use crate::server_message::Result::SubscribeError;

pub(crate) fn connect(ip_address: String, username: String, port: u16) {
    let stream = TcpStream::connect("{ip_address}:{port}".replace("{ip}", &ip_address).replace("{port}", &port.to_string()));
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

    if length > 4096 {
        return "".to_string();
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
    let last_leaderboard= &mut Vec::<PublicPlayer>::new();

    while is_connection_opened {
        let message = read_message(&stream);
        // println!("Message: {:?}", message);

        if message == "" {
            continue;
        }

        let message_json = serde_json::from_str(&message).unwrap();
        match message_json {
            ServerMessage::Welcome(_) => {
                // println!("Welcome: {:?}", welcome);
                send_username(&stream, &username);
            }
            ServerMessage::SubscribeResult(subscribe_result) => {
                // println!("Subscribe result: {:?}", subscribe_result);
                match subscribe_result {
                    SubscribeError(subscribe_error) => {
                        println!("Subscribe error: {:?}", subscribe_error);
                        stream.shutdown(std::net::Shutdown::Both).unwrap();
                    }
                    _ => {
                        println!("Connected to the server");
                    }
                }
            }
            ServerMessage::PublicLeaderBoard(public_leader_board) => {
                //println!("PublicLeaderBoard: {:?}", public_leader_board);
                *last_leaderboard = public_leader_board;
            }
            ServerMessage::RoundSummary(_) => {
                //println!("RoundSummary: {:?}", round_summary);
            }
            ServerMessage::Challenge(challenge) => {
                // println!("Challenge: {:?}", challenge);
                match challenge {
                    MD5HashCash(md5_hash_cash) => {
                        // println!("MD5HashCash: {:?}", md5_hash_cash);
                        let hashcash = HashCash::new(md5_hash_cash);
                        let hashcash_result = &HashCash::solve(&hashcash);
                        let hashcash_output = ChallengeOutput::MD5HashCash(MD5HashCashOutput {
                            seed: hashcash_result.seed,
                            hashcode: hashcash_result.hashcode.to_string(),
                        });
                        let challenge_result = format_challenge_result(hashcash_output, last_leaderboard, username.clone());
                        send_message(&stream, &serde_json::to_string(&challenge_result).unwrap());
                    }
                    RecoverSecret(recover_secret) => {
                        // println!("RecoverSecret: {:?}", recover_secret);
                        let recover_secret = Recover::new(recover_secret);
                        let recover_secret_result = &Recover::solve(&recover_secret);
                        let recover_secret_output = ChallengeOutput::RecoverSecret(RecoverSecretOutput {
                            secret_sentence: recover_secret_result.secret_sentence.to_string()
                            //secret_sentence: "C'est chou".to_string()
                        });
                        let challenge_result = format_challenge_result(recover_secret_output, last_leaderboard, username.clone());
                        send_message(&stream, &serde_json::to_string(&challenge_result).unwrap());
                    }
                }
            }
            ServerMessage::EndOfGame(_) => {
                // println!("EndOfGame: {:?}", end_of_game);
                println!("Game over, closing server connection...");
                is_connection_opened = false;


                match  stream.shutdown(std::net::Shutdown::Both) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        }
    }
}

fn format_challenge_result(challenge_output: ChallengeOutput, leaderboard: &mut Vec<PublicPlayer>, username: String) -> ClientMessage {
    return ClientMessage::ChallengeResult(ChallengeResult {
        answer: challenge_output,
        next_target: compute_next_target(leaderboard, username.clone())
    });
}

fn compute_next_target(leaderboard: &mut Vec<PublicPlayer>, username: String) -> String {
    let leaderboard = leaderboard;
    leaderboard.sort_by(|a, b| b.score.cmp(&a.score));
    let next_target = leaderboard.iter().filter(|public_player| public_player.name != username).nth(0).unwrap();
    return next_target.name.to_string();
}

#[test]
fn test_transform_u32_to_array_of_u8() {
    let x: u32 = 8;
    let result: [u8; 4] = [0, 0, 0, 8];
    assert_eq!(transform_u32_to_array_of_u8(x), result);
}

#[test]
fn test_transform_array_of_u8_to_u32() {
    let x: [u8; 4] = [0, 0, 0, 8];
    assert_eq!(transform_array_of_u8_to_u32(x), 8);
}
