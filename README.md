# password-manager-cs128h

Project Outline: Rust CLI Password Manager

Project Structure

Modules:

      - `main.rs`: Entry point, command-line argument handling
      
      - `commands.rs`: Define commands (login, create, delete, list, update, etc.)
      
      - `storage.rs`: Handles data storage and retrival
      
      - `utils.rs`: Utility functions (encryption , domain parsing, etc.)

Core Features

Command Line Interface (CLI)
"Commands: ☑️

      - `create {domain} {username} {password}`: Save new credentials for a domain
      
      - `delete {domain}`: Remove credentials for a specified domain
      
      - `update {domain} {username} {password}`: Update existing credentials
      
      - `generate {domain} {username} {length}`: Create a new login with a generated password
      
      - `login {domain}`: Retrieve and autofill credentials on the target website
      
      - `list`: Display all saved domains and usernames
      
      - `lock`: Prompts for master password
      
      - `help`: List all commands 
      
      - `exit` or `quit`: Exit the program"

CLI Design: ☑️
      - Use the `clap` crate for command-line parsing and argument handling
      - Integrate clear and user-friendly error handling messages

Secure Storage of Credentials

Encryption: ☑️
Use AES-256 encryption with a master password to encrypt credentials
Leverage the `argon2` crate for secure encryption and decryption

Master-Password Hashing: ☑️
Hash and securely store the master password using sha256

Data Storage: ☑️
Store data in an encrypted csv file

Secure Master Key Storage: ☑️
Prompt for the master password on every use, avoiding caching it in memory for long sessions
Optionally integrate with system keychain services for enhanced security

Domain-based Credential Retrieval: ☑️

Parse and standardize domains so the user doesn’t need exact URLs (`example.com` matches `https://example.com/*`)
Regex or domain-parsing crate to consistently handle subdomains and protocols

Additional Features

Password Generation: ☑️
Add a `generate` command to create logins with secure passwords of customizable length
 
Security Features: ☑️
Implement a command `lock` that requires re-authentication with master key
Automatic clipboard clearing after a set time when copying passwords
