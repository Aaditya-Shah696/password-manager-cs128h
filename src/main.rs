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

fn process_command(_input: &str) -> Result<String, String> {
    // Implement command processing logic here
    Ok("Command processed".to_string())
}
