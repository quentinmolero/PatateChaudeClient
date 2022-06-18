mod connection_handler;
mod client_message;
mod server_message;
mod challenge_message;

fn main() {
    println!("Hello, world!");
    connection_handler::connect();
}
