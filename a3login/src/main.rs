use std::{
    env, error::Error, ffi::OsString, io::{self, Write}, process
};
use argon2::{
    password_hash::PasswordVerifier, Argon2, PasswordHash
};

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;

    let login_input = LoginInfo::new();

    let mut login_success: bool = false;
    for result in rdr.records() {
        let record = result?;

        let username = &record[0];
        let password_hash  = PasswordHash::new(&record[1]).unwrap(); // fix the unwrap here

        
        if username == login_input.username.trim() &&  Argon2::default().verify_password(login_input.password.trim().as_bytes(), &password_hash).is_ok() {
            println!{"Access granted!"};
            login_success = true;
        }
    }
    if login_success == false {
        println!("Error! Access denied!");
    }
    Ok(())
}


fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read user input");

    input
}

struct LoginInfo {
    username: String,
    password: String
}

impl LoginInfo {
    fn new() -> LoginInfo {
        print!("Enter username: ");
        io::stdout().flush().unwrap(); // need to fix the unwrap here
        let username = read_input();

        print!("Enter password: ");
        io::stdout().flush().unwrap();
        let password = read_input();

        LoginInfo { username, password }
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}