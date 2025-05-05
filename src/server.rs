/* TCP server logic, handles client connections */

use std::{
    io::{BufRead, BufReader},
    net::TcpListener,
};

pub fn start_server() {
    println!("Starting the server side ....");
    connect();
}
fn connect(){
    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to bind port");
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("eastablished connection with {}", addr);
                let f = BufReader::new(stream);

                for line in f.lines(){
                    match line {
                        Ok(text) => println!(" Client says: {}", text),
                        Err(e) => println!("Error : {}", e),
                    }
                }
                }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}