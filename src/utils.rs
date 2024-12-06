use addr::parse_domain_name;

use linked_hash_map::LinkedHashMap;
use std::error::Error;
use csv::{ReaderBuilder, WriterBuilder};
use crate::LoginDatabase;

pub fn parse_domain(url: &str) -> Option<String> {
    let domain = url.trim_start_matches("http://")
                    .trim_start_matches("https://")
                    .trim_start_matches("www.");

    let end = domain.find('/').unwrap_or(domain.len());
    let domain = &domain[..end];

    parse_domain_name(domain)
        .ok()
        .and_then(|parsed| parsed.root().map(String::from))
}   

pub fn read_csv(file_path: &str) -> Result<LoginDatabase, Box<dyn Error>> {
    let mut logins = LinkedHashMap::new();
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;

    for result in reader.records() {
        let record = result?;
        if record.len() == 3 {
            logins.insert(
                record[0].to_string(),
                (record[1].to_string(), record[2].to_string()),
            );
        }
    }

    Ok(logins)
}

pub fn write_csv(file_path: &str, logins: &LoginDatabase) -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;

    // Write in the correct order: domain, username, password
    for (domain, (username, password)) in logins {
        writer.write_record(&[domain, username, password])?;
    }
    writer.flush()?;

    Ok(())
}
