use anyhow::{Context, Result};
use log::{info, warn};
use std::fs::File;
use std::io::{BufRead, BufReader};
use url::Url;

pub fn parse_links_file(filepath: &str) -> Result<Vec<(String, Option<String>)>> {
    let file =
        File::open(filepath).with_context(|| format!("Failed to open links file: {}", filepath))?;

    let reader = BufReader::new(file);
    let mut links = Vec::new();

    for line_result in reader.lines() {
        let line = line_result
            .with_context(|| format!("Error reading line from {}", filepath))?
            .trim()
            .to_string();

        // Skip empty lines and full-line comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Split line into potential URL and comment
        let parts: Vec<&str> = line.splitn(2, '#').collect();
        let url_str = parts[0].trim();

        // Validate URL
        match Url::parse(url_str) {
            Ok(_) => {
                let comment = if parts.len() > 1 {
                    Some(parts[1].trim().to_string())
                } else {
                    None
                };

                info!(
                    "Found link: {} {}",
                    url_str,
                    comment.clone().unwrap_or_default()
                );
                links.push((url_str.to_string(), comment));
            }
            Err(_) => {
                warn!("Invalid URL: {}", url_str);
            }
        }
    }

    Ok(links)
}
