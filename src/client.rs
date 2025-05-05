use std::io::Write;
/* TCP client logic, interacts with server */
use std::net::TcpStream;
// use std::env;
// use std::io::{Self, Write};

pub fn start_client(){
    println!("Client side started ....");
    // let args : Vec<String> = env::args().collect();
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("error connection client side ...");
    stream.write_all(b"Balance 123\nanother line\n").expect("error sending the buf.");
}