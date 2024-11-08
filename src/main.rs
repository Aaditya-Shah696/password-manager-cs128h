mod commands;
mod storage;
mod utils;
mod integration;
use clap::Parser;
use std::process::Command;
use std::env;
use std::io::Write;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    in_new_window: bool,
}

fn main() {
    let cli = Cli::parse();
    if env::var("PASSWORD_MANAGER_NEW_WINDOW").is_ok() || cli.in_new_window {
        // Run the interactive shell if in the new terminal instance
        run_interactive_shell();
    } else {
        // Open a new terminal window and set the environment variable
        let program_path = env::current_exe().unwrap();
        let program_path_str = program_path.to_str().unwrap();

        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/c", "start", "cmd", "/k", 
                        &format!("set PASSWORD_MANAGER_NEW_WINDOW=1 && {} --in-new-window", program_path_str)])
                .spawn()
                .expect("Failed to open new terminal window on Windows");
        } else if cfg!(target_os = "macos") {
            Command::new("open")
                .args(&[
                    "-a", "Terminal",
                    program_path_str,
                    "--args", "--in-new-window"
                ])
                .env("PASSWORD_MANAGER_NEW_WINDOW", "1")
                .spawn()
                .expect("Failed to open new terminal window on macOS");
        } else {
            eprintln!("Unsupported OS for opening a new terminal window.");
        }
        println!("Opened a new terminal window. This window will close.");
    }
}

fn run_interactive_shell() {
    println!("Welcome to the password manager. Type 'exit' or 'quit' to end the program.");
    
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" || input == "quit" {
            println!("Exiting the program...");
            break;
        }

        // Process the command here
        match process_command(input) {
            Ok(output) => println!("{}", output),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}


fn process_command(input: &str) -> Result<String, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Err("No command entered.".to_string());
    }

    /*
    Commands:
      - `create {username} {password} {domain}`: Save new credentials for a domain
      - `delete {domain}`: Remove credentials for a specified domain
      - `update {username} {password} {domain}`: Update existing credentials
      - `login {domain}`: Retrieve and autofill credentials on the target website
      - `list`: Display all saved domains and usernames
      - `exit` or `quit`: Exit the program
    */
    match parts[0] {
        "create" => {
            if parts.len() != 4 {
                Err("Usage: create <username> <password> <domain>".to_string())
            } else {
                commands::create(parts[1], parts[2], parts[3])
            }
        },
        "delete" => {
            if parts.len() != 2 {
                Err("Usage: delete <domain>".to_string())
            } else {
                commands::delete(parts[1])
            }
        },
        "update" => {
            if parts.len() != 4 {
                Err("Usage: update <username> <password> <domain>".to_string())
            } else {
                commands::update(parts[1], parts[2], parts[3])
            }
        },
        "login" => {
            if parts.len() != 2 {
                Err("Usage: login <domain>".to_string())
            } else {
                commands::login(parts[1])
            }
        },
        "list" => {
            if parts.len() != 1 {
                Err("Usage: list".to_string())
            } else {
                commands::list()
            }
        },
        _ => Err(format!("Unknown command: {}", parts[0])),
    }
}
