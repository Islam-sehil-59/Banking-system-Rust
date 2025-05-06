use std::env;
use bank::client::start_client;
use bank::server::start_server;
use bank::net::auth::client_connect;

fn main(){
    let args: Vec<String> = env::args().collect();
    let error_dis: &str = "Error occured\n--help \t\tCkeck help and manuel";
    if args.len() != 2 {
        println!("{}",error_dis);
    }
    if args.len() == 3 && args[1] == "client" && args[2] == "connect" {
        let _ = client_connect();
    }
    match args[1].as_str() {
        "client" => {start_client();},
        "server" => {start_server();},
        "help"  => {
            println!("Usage: cargo run -- client connect [ID]:[Pass]")
        }
        _ => println!("{}",error_dis),
    }
}