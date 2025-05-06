use std::io::{stdin, stdout, Write};


pub fn client_connect(){
    println!("user_id: ");
    let mut user_id = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut user_id).expect("invalide user_id");
    let user_id = user_id.trim().to_string();
    println!("passwd : ");
    stdout().flush().unwrap();
    let mut passwd = String::new();
    stdin().read_line(&mut passwd).expect("invalide passwd.");
    let passwd = passwd.trim();
    println!("user_id = {}\npasswd = {}",user_id ,passwd);

}