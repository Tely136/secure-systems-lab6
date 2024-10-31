use std::{
    env, error::Error, ffi::OsString, io::{self, Write}, process
};
use argon2::{
    password_hash::PasswordVerifier, Argon2, PasswordHash
};
use std::path::Path;


fn run() -> Result<(), Box<dyn Error>> {
    // Get first argument which should be the path to the login databse
    let database_path = get_first_arg()?;

    // Make sure the path exists, display an error mesage if it doesn't
    _ = check_existence(&database_path);

    // Create new login struct wuth user input
    let login_input = LoginInfo::new();

    // Call the verify login function
    _ = verify_login(login_input.username.trim(), login_input.password.trim(), database_path);

    Ok(())
}


// Function to check that the path to the login database exists
fn check_existence(path: &OsString) {
    if Path::new(&path).exists() {}
    else {
        eprintln!("Error! Password database not found!");
        std::process::exit(1);
    }
}

// Function to verify username and password against database
fn verify_login(input_username: &str, input_password: &str, database_path:OsString) -> Result<bool, Box<dyn Error>> {

    // create csv reader from database path
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(database_path)
        .unwrap_or_else(|_err| {
            eprintln!("error reading csv file");
            std::process::exit(1);
    });

    // Initialize variable to false then loop over records in csv reader
    let mut login_success: bool = false;
    for result in rdr.records() {
        let record = result?;

        let username = &record[0];

        // Convert string to a PasswordHash object
        let password_hash  = PasswordHash::new(&record[1]).unwrap_or_else(|err| {
            eprintln!("Failed to parse password hash: {:?}", err);
            std::process::exit(1); 
        });
        
        // Check if input username and input password match current set of credentials in csv file
        if username == input_username &&  Argon2::default().verify_password(input_password.as_bytes(), &password_hash).is_ok() {
            // If they match then print success message and change variable to true
            println!{"Access granted!"};
            login_success = true;
        }
    }

    // If the entire databse has been looked at and login_success is still false, print error message
    if login_success == false {
        println!("Error! Access denied!");
    }
    Ok(login_success)
}


// Function to return first argument passed when this function is run
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

// Function to read user input from the terminal
fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap_or_else(|err| {
            eprintln!("Error reading input: {}", err);
            std::process::exit(1);
        });

    input
}

struct LoginInfo {
    username: String,
    password: String
}

impl LoginInfo {
    fn new() -> LoginInfo {
        print!("Enter username: ");
        _ = io::stdout().flush();
        let username = read_input();

        print!("Enter password: ");
        _ = io::stdout().flush();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_login() {
        let username = "admin";
        let password = "hunter2";

        assert_eq!(true, verify_login(&username, &password, OsString::from("db.csv")).unwrap());
    }

    #[test]
    fn test_incorrect_login() {
        let username = "admin";
        let password = "hunter1";

        assert_eq!(false, verify_login(&username, &password, OsString::from("db.csv")).unwrap());
    }
    
    #[test]
    fn test_missing_user() {
        let username = "root";
        let password = "hunter2";

        assert_eq!(false, verify_login(&username, &password, OsString::from("db.csv")).unwrap());
    }
}