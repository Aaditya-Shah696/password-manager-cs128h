mod commands;
mod utils;
mod storage;
use clap::Parser;
use std::process::Command;
use std::env;
use std::fs;
use std::io::Write;
use linked_hash_map::LinkedHashMap;
use sha256::digest;
use std::sync::Mutex;
use std::sync::Arc;

type Credentials = (String, String);
type LoginDatabase = LinkedHashMap<String, Credentials>;

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
                .args(&["-a", "Terminal",program_path_str,"--args", "--in-new-window"])
                .env("PASSWORD_MANAGER_NEW_WINDOW", "1")
                .spawn()
                .expect("Failed to open new terminal window on macOS");
        } else {
            eprintln!("Unsupported OS for opening a new terminal window.");
        }
        println!("Opened a new terminal window. This window will close.");
    }
}

fn mp_lock() -> String {
    let master_pass = fs::read_to_string("masterpassword.txt")
        .expect("Should have been able to read the file");
    
    loop {
        println!("Please enter Master Password (It's `password`)");
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if digest(input.clone()) != master_pass {
            println!("Incorrect Password.\n")
        } else {
            println!("\nWelcome to the password manager. Type 'exit' or 'quit' to end the program, or 'help' to see a list of commands");
            return input;
        }
    }
}

fn run_interactive_shell() {

    let master_password = mp_lock();
    let master_key = Arc::new(Mutex::new(utils::derive_key(&master_password)));
    
    let file_path = "logins.csv";
    let mut logins = match utils::read_csv(file_path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Error reading CSV file: {}. Starting with an empty database.", e);
            LoginDatabase::new()
        }
    };

    loop {
        print!("\n> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" || input == "quit" {
            println!("Exiting the program...");
            break;
        }

        // Process the command here
        match process_command(input, &mut logins, &master_key) {
            Ok(output) => {
                println!("{}", output);
                // Only save if the command was not "list"
                if !input.trim().starts_with("list") {
                    if let Err(e) = utils::write_csv(file_path, &logins) {
                        eprintln!("Error writing to CSV file: {}\n", e);
                    }
                }
            },
            Err(e) => eprintln!("Error: {}", e),
        }
        
    }
}


fn process_command(input: &str, logins: &mut LoginDatabase, master_key: &Arc<Mutex<[u8; 32]>>) -> Result<String, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Ok("".to_string());
    }

    
    match parts[0] {
        "create" => {
            if parts.len() != 4 {
                Err("Usage: create <domain> <username> <password>".to_string())
            } else {
                commands::create(parts[1], parts[2], parts[3], logins, master_key)
            }
        },
        "delete" => {
            if parts.len() != 2 {
                Err("Usage: delete <domain>".to_string())
            } else {
                commands::delete(parts[1], logins)
            }
        },
        "update" => {
            if parts.len() != 4 {
                Err("Usage: update <domain> <username> <password>".to_string())
            } else {
                commands::update(parts[1], parts[2], parts[3], logins, master_key)
            }
        },
        "login" => {
            if parts.len() != 2 {
                Err("Usage: login <domain>".to_string())
            } else {
                commands::login(parts[1], logins, master_key)
            }
        },
        "list" => {
            if parts.len() != 1 {
                Err("Usage: list".to_string())
            } else {
                commands::list(logins)
            }
        },
        "generate" => {
            if parts.len() != 4 {
                Err("Usage: generate <domain> <username> <length>".to_string())
            } else {
                let length = parts[3].parse::<usize>().map_err(|_| "Invalid length parameter".to_string())?;
                commands::generate(parts[1], parts[2], length, logins, master_key)
            }
        },
        "help" => {
            if parts.len() != 1 {
                Err("Usage: help".to_string())
            } else {
                commands::help()
            }
        },
        "lock" => {
            if parts.len() != 1 {
                Err("Usage: lock".to_string())
            } else {
                println!("");
                mp_lock();
                Ok(format!(""))
            }
        }
        _ => Err(format!("Unknown command: {}", parts[0])),
    }
}
