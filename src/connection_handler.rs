use std::io::{Read, Write};
use std::net::{TcpStream};
use serde_json;
use crate::challenge::Challenge;
#[allow(unused_imports)]
use crate::challenge_message::{ChallengeOutput, ChallengeResult, MD5HashCashInput, MD5HashCashOutput, MonstrousMazeOutput, RecoverSecretOutput};
use crate::challenge_message::Challenge::{MD5HashCash, MonstrousMaze, RecoverSecret};
use crate::challenge_monstrous_maze::monstrous_maze_challenge::Monstrous;
use crate::challenge_recover_secret::recover_secret_challenge::Recover;

use crate::client_message::{ClientMessage, Subscribe};
use crate::md5cash_challenge::HashCash;
#[allow(unused_imports)]
use crate::server_message::{PublicPlayer, ServerMessage, Welcome};
use crate::server_message::Result::SubscribeError;

pub(crate) fn connect(ip_address: String, username: String, port: u16) {
    let stream = TcpStream::connect("{ip_address}:{port}".replace("{ip_address}", &ip_address).replace("{port}", &port.to_string()));
    match stream {
        Ok(stream) => {
            say_hello(&stream);
            listen_from_stream(&stream, username);
        }
        Err(err) => panic!("Cannot connect : {}", err),
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

fn format_string_to_json(string: &ClientMessage) -> String {
    match serde_json::to_string(&string) {
        Ok(string) => {
            return string
        }
        Err(err) => {
            panic!("Cannot format string to json : {}", err)
        }
    }
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

fn format_json_to_message(message: String) -> ServerMessage {
    match serde_json::from_str(&message) {
        Ok(server_message) => {
            return server_message
        }
        Err(err) => {
            panic!("Cannot format json to message : {}", err)
        }
    }
}

fn say_hello(stream: &TcpStream) {
    send_message(stream, "\"Hello\"");
}

fn send_username(stream: &TcpStream, username: &str) {
    let message = ClientMessage::Subscribe(Subscribe {
        name: username.to_string(),
    });
    send_message(stream, &format_string_to_json(&message));
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

        let message_json = format_json_to_message(message);

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
                        close_socket_connection(stream);
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
                        send_message(&stream, &format_string_to_json(&challenge_result));
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
                        send_message(&stream, &format_string_to_json(&challenge_result));
                    }
                    MonstrousMaze(monstrous_maze_input) => {
                        //println!("Monstrous Maze: {:?}", monstrous_maze_input);
                        let monstrous = Monstrous::new(monstrous_maze_input);
                        let monstrous_result = &Monstrous::solve(&monstrous);
                        let monstrous_output_result = ChallengeOutput::MonstrousMaze(MonstrousMazeOutput {
                            path: monstrous_result.path.to_string(),
                        });
                        let challenge_result = format_challenge_result(monstrous_output_result, last_leaderboard, username.clone());
                        send_message(&stream, &format_string_to_json(&challenge_result));
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
    let next_target = leaderboard.iter().filter(|public_player| public_player.name != username).nth(0);
    return match next_target {
        Some(public_player) => {
            public_player.name.to_string()
        }
        None => {
            "".to_string()
        }
    }
}

fn close_socket_connection(stream: &TcpStream) {
    match stream.shutdown(std::net::Shutdown::Both) {
        Ok(_) => {
            println!("Connection closed");
        }
        Err(err) => {
            println!("Couldn't close connection: {}", err);
        }
    }
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

#[test]
fn test_format_string_to_json_subscribe() {
    let message = ClientMessage::Subscribe(Subscribe {
        name: "test".to_string(),
    });
    let result = "{\"Subscribe\":{\"name\":\"test\"}}";
    assert_eq!(format_string_to_json(&message), result);
}

#[test]
fn test_format_string_to_json_challenge_result() {
    let message = ClientMessage::ChallengeResult(ChallengeResult {
        answer: ChallengeOutput::MD5HashCash(MD5HashCashOutput {
            seed: 1,
            hashcode: "test".to_string(),
        }),
        next_target: "test".to_string(),
    });
    let result = "{\"ChallengeResult\":{\"answer\":{\"MD5HashCash\":{\"seed\":1,\"hashcode\":\"test\"}},\"next_target\":\"test\"}}";
    assert_eq!(format_string_to_json(&message), result);
}

#[test]
fn test_format_json_to_message_welcome() {
    let message = "{\"Welcome\":{\"version\":1}}".to_string();
    let result = ServerMessage::Welcome(Welcome {
        version: 1,
    });
    assert_eq!(format_json_to_message(message), result);
}

#[test]
fn test_format_json_to_message_challenge() {
    let result = ServerMessage::Challenge(MD5HashCash(MD5HashCashInput{
        complexity: 0,
        message: "test".to_string(),
    }));
    let message = "{\"Challenge\":{\"MD5HashCash\":{\"complexity\":0,\"message\":\"test\"}}}".to_string();
    assert_eq!(format_json_to_message(message), result);
}

#[test]
fn test_format_challenge_result_md5_hash_cash() {
    let mut leaderboard = Vec::new();
    leaderboard.push(PublicPlayer {
        name: "test".to_string(),
        stream_id: "".to_string(),
        score: 10,
        steps: 0,
        is_active: false,
        total_used_time: 0.0
    });
    leaderboard.push(PublicPlayer {
        name: "test2".to_string(),
        stream_id: "".to_string(),
        score: 5,
        steps: 0,
        is_active: false,
        total_used_time: 0.0
    });
    let message = format_challenge_result(ChallengeOutput::MD5HashCash(MD5HashCashOutput {
        seed: 1,
        hashcode: "test".to_string(),
    }), &mut leaderboard, "test2".to_string());
    let result = ClientMessage::ChallengeResult(ChallengeResult {
        answer: ChallengeOutput::MD5HashCash(MD5HashCashOutput {
            seed: 1,
            hashcode: "test".to_string(),
        }),
        next_target: "test".to_string(),
    });
    assert_eq!(message, result);
}

#[test]
fn test_format_challenge_result_md5_hash_cash_but_i_have_the_highest_score() {
    let mut leaderboard = Vec::new();
    leaderboard.push(PublicPlayer {
        name: "test".to_string(),
        stream_id: "".to_string(),
        score: 10,
        steps: 0,
        is_active: false,
        total_used_time: 0.0
    });
    leaderboard.push(PublicPlayer {
        name: "test2".to_string(),
        stream_id: "".to_string(),
        score: 5,
        steps: 0,
        is_active: false,
        total_used_time: 0.0
    });
    let message = format_challenge_result(ChallengeOutput::MD5HashCash(MD5HashCashOutput {
        seed: 1,
        hashcode: "test".to_string(),
    }), &mut leaderboard, "test".to_string());
    let result = ClientMessage::ChallengeResult(ChallengeResult {
        answer: ChallengeOutput::MD5HashCash(MD5HashCashOutput {
            seed: 1,
            hashcode: "test".to_string(),
        }),
        next_target: "test2".to_string(),
    });
    assert_eq!(message, result);
}

#[test]
fn test_compute_next_target() {
    let mut leaderboard = Vec::new();
    leaderboard.push(PublicPlayer {
        name: "test".to_string(),
        stream_id: "".to_string(),
        score: 1,
        steps: 0,
        is_active: false,
        total_used_time: 0.0
    });
    leaderboard.push(PublicPlayer {
        name: "test2".to_string(),
        stream_id: "".to_string(),
        score: 2,
        steps: 0,
        is_active: false,
        total_used_time: 0.0
    });
    leaderboard.push(PublicPlayer {
        name: "test3".to_string(),
        stream_id: "".to_string(),
        score: 3,
        steps: 0,
        is_active: false,
        total_used_time: 0.0
    });
    assert_eq!(compute_next_target(&mut leaderboard, "test".to_string()), "test3".to_string());
}

#[test]
fn test_compute_next_target_if_i_have_the_highest_score() {
    let mut leaderboard = Vec::new();
    leaderboard.push(PublicPlayer {
        name: "test".to_string(),
        stream_id: "".to_string(),
        score: 1,
        steps: 0,
        is_active: false,
        total_used_time: 0.0
    });
    leaderboard.push(PublicPlayer {
        name: "test2".to_string(),
        stream_id: "".to_string(),
        score: 2,
        steps: 0,
        is_active: false,
        total_used_time: 0.0
    });
    leaderboard.push(PublicPlayer {
        name: "test3".to_string(),
        stream_id: "".to_string(),
        score: 3,
        steps: 0,
        is_active: false,
        total_used_time: 0.0
    });
    assert_eq!(compute_next_target(&mut leaderboard, "test3".to_string()), "test2".to_string());
}
