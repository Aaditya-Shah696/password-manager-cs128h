mod commands;
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

    if !cli.in_new_window {
        // Open a new terminal window
        let program_path = env::current_exe().unwrap();
        let program_path_str = program_path.to_str().unwrap();

        Command::new("cmd")
            .args(&["/c", "start", "cmd", "/k", 
                    &format!("{} --in-new-window", program_path_str)])
            .spawn()
            .expect("Failed to open new terminal window");

        println!("Opened a new terminal window. This window will close.");
    } else {
        // This is the new window, run interactive shell
        run_interactive_shell();
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

/*
Commands:
      - `create {username} {password} {domain}`: Save new credentials for a domain
      - `delete {domain}`: Remove credentials for a specified domain
      - `update {username} {password} {domain}`: Update existing credentials
      - `login {domain}`: Retrieve and autofill credentials on the target website
      - `list`: Display all saved domains and usernames
      - `exit` or `quit`: Exit the program
*/
fn process_command(input: &str) -> Result<String, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Err("No command entered.".to_string());
    }

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
