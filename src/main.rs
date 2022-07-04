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
        args.value_of("ip").unwrap().to_string()
    } else {
        "localhost".to_string()
    }
}

fn get_username(args: &ArgMatches) -> String {
    if args.is_present("username") {
        args.value_of("username").unwrap().to_string()
    } else {
        "Player".to_string() + &SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string()
    }
}

fn get_port(args: &ArgMatches) -> u16 {
    if args.is_present("port") {
        args.value_of("port").unwrap().parse().unwrap()
    } else {
        7878
    }
}
