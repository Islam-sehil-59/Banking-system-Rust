/* Entry point, starts server or client based on args */
use std::env;
mod client;
use client::start_client;
mod server;
use server::start_server;


fn main() {
    let args: Vec<String>= env::args().collect();
    // dbg!(&args);
    // println!("{:?}", args);
    match args[1].as_str(){
        "server" => {
            println!("starting server ...");
            start_server();
        },
        "client" => {
            println!("starting client ....");
            start_client();
        },
        _ => {
            println!("-- server\t\truns server ...\n-- client\t\truns client side ....");
            dbg!(args);
        }
    }
}
