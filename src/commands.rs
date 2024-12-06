use crate::LoginDatabase;
use crate::utils;

use clipboard::{ClipboardContext, ClipboardProvider};

pub fn create(domain: &str, username: &str, password: &str, logins: &mut LoginDatabase) -> Result<String, String> {
    let parsed_domain = utils::parse_domain(domain)
        .ok_or_else(|| format!("Invalid domain format: {}", domain))?;
    
    if logins.contains_key(&parsed_domain) {
        return Err(format!("An account for {} already exists", parsed_domain));
    }
    
    logins.insert(parsed_domain.clone(), (username.to_string(), password.to_string()));
    
    match utils::write_csv("logins.csv", logins) {
        Ok(_) => Ok(format!("Account created successfully for {}", parsed_domain)),
        Err(e) => {
            logins.remove(&parsed_domain);
            Err(format!("Failed to save account: {}", e))
        }
    }
}

pub fn delete(domain: &str, logins: &mut LoginDatabase) -> Result<String, String> {
    let parsed_domain = utils::parse_domain(domain)
        .ok_or_else(|| format!("Invalid domain format: {}", domain))?;
        
    if parsed_domain.to_lowercase() == "domain" {
        return Err("Cannot delete the header row".to_string());
    }

    if logins.remove(&parsed_domain).is_some() {
        match utils::write_csv("logins.csv", logins) {
            Ok(_) => Ok(format!("Account for {} deleted successfully", parsed_domain)),
            Err(e) => Err(format!("Failed to delete account: {}", e))
        }
    } else {
        Err(format!("No account found for {}", parsed_domain))
    }
}

pub fn update(domain: &str, username: &str, password: &str, logins: &mut LoginDatabase) -> Result<String, String> {
    let parsed_domain = utils::parse_domain(domain)
        .ok_or_else(|| format!("Invalid domain format: {}", domain))?;
    
    if parsed_domain.to_lowercase() == "domain" {
        return Err("Cannot update the header row".to_string());
    }

    if logins.contains_key(&parsed_domain) {
        logins.insert(parsed_domain.clone(), (username.to_string(), password.to_string()));
        
        match utils::write_csv("logins.csv", logins) {
            Ok(_) => Ok(format!("Account updated successfully for {}", parsed_domain)),
            Err(e) => Err(format!("Failed to update account: {}", e))
        }
    } else {
        Err(format!("No account found for {}", parsed_domain))
    }
}

pub fn login(domain: &str, logins: &LoginDatabase) -> Result<String, String> {
    let parsed_domain = utils::parse_domain(domain)
        .ok_or_else(|| format!("Invalid domain format: {}", domain))?;

    if parsed_domain.to_lowercase() == "domain" {
        return Err("Cannot login to header row".to_string());
    }

    if let Some((username, password)) = logins.get(&parsed_domain) {
        // Try to get clipboard context
        let mut ctx = ClipboardContext::new()
            .map_err(|e| format!("Failed to access clipboard: {}", e))?;
        
        // Copy username and password to clipboard
        let credentials = format!("{}\n{}", username, password);
        ctx.set_contents(credentials)
            .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;
            
        Ok(format!("Credentials for {} copied to clipboard", parsed_domain))
    } else {
        Err(format!("No account found for {}", parsed_domain))
    }
}


pub fn list(logins: &LoginDatabase) -> Result<String, String> {
    if logins.is_empty() {
        Ok("No accounts stored".to_string())
    } else {
        let account_list: Vec<String> = logins
            .iter()
            .map(|(domain, (username, _))| format!("{}: {}", domain, username))
            .collect();
        Ok(format!("List of accounts:\n{}", account_list.join("\n")))
    }
}
