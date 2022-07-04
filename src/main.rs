mod connection_handler;
mod client_message;
mod server_message;
mod challenge_message;
mod md5cash_challenge;
mod challenge;
mod recover_secret;
mod recover_secret_challenge;

use std::time::{SystemTime, UNIX_EPOCH};
use clap::{App, Arg, ArgMatches};

fn main() {
    let args = App::new("patate_chaude_client")
        .version("1.0")
        .arg(Arg::with_name("ip")
            .short("i".parse().unwrap())
            .help("Sets the ip of the serveur, default is 127.0.0.1")
            .takes_value(true))
        .arg(Arg::with_name("port")
            .short("p".parse().unwrap())
            .help("Sets the port")
            .takes_value(true))
        .arg(Arg::with_name("username")
             .short("u".parse().unwrap())
             .help("Sets the username")
             .takes_value(true))
        .get_matches();
    connection_handler::connect(get_ip_address(&args), get_username(&args), get_port(&args));
}

fn get_ip_address(args: &ArgMatches) -> String {
    if args.is_present("ip") {
        match args.value_of("ip") {
            Some(ip) => {
                return ip.to_string()
            }
            None => {
                "localhost".to_string()
            }
        }
    } else {
        "localhost".to_string()
    }
}

fn get_username(args: &ArgMatches) -> String {
    return if args.is_present("username") {
        match args.value_of("username") {
            Some(username) => {
                username.to_string()
            }
            None => {
                get_default_player_name()
            }
        }
    } else {
        get_default_player_name()
    }
}

fn get_default_player_name() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(time) => {
            return "Player".to_string() + &time.as_millis().to_string();
        }
        Err(error) => {
            panic!("{:?}", error);
        }
    }
}

fn get_port(args: &ArgMatches) -> u16 {
    return match args.value_of("port") {
        Some(port) => {
            match port.parse() {
                Ok(port) => {
                    return port
                }
                Err(error) => {
                    panic!("{:?}", error);
                }
            }
        }
        None => {
            7878
        }
    }
}
