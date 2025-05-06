use std::{error::Error, io::{stdin, stdout, Write}};
use rusqlite::{
    params,Connection,Result,
};
use rpassword::prompt_password;


pub fn client_connect() -> Result<(), Box<dyn Error>>{
    
    print!("user_id: ");
    let mut user_id = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut user_id).expect("invalide user_id");
    let user_id = user_id.trim().to_string();

    // print!("passwd : ");
    // stdout().flush().unwrap();
    // let mut passwd = String::new();
    // stdin().read_line(&mut passwd).expect("invalide passwd.");
    // let passwd = passwd.trim();
    
    let passwd = prompt_password("passwd : ").expect("err reading passwd");
    println!("user_id = {}\npasswd = {}",user_id ,passwd);

    let conn = Connection::open("users.db")?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id TEXT NOT NULL,
        passwd TEXT NOT NULL,
        )",
        [],
    )?;
    
    Ok(())
}
