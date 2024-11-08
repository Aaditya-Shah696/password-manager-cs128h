use crypto::digest::Digest;
use crypto::sha2::Sha256;

use addr::parse_domain_name;

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(password);
    hasher.result_str()
}

fn parse_domain(url: &str) -> Option<String> {
    let domain = url.trim_start_matches("http://")
                    .trim_start_matches("https://")
                    .trim_start_matches("www.");

    let end = domain.find('/').unwrap_or(domain.len());
    let domain = &domain[..end];

    parse_domain_name(domain)
        .ok()
        .and_then(|parsed| parsed.root().map(String::from))
}