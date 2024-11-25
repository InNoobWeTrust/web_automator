use anyhow::{Context, Result};
use fantoccini::cookies::Cookie as FantocciniCookie;
use fantoccini::Client;
use log::{info, warn};
use std::fs::File;
use std::io::BufReader;
use url::Url;

use crate::models::Cookie;

pub async fn load_cookies_for_domain(
    client: &mut Client,
    cookie_file: &str,
    target_url: &str,
) -> Result<bool> {
    let url = Url::parse(target_url)?;
    let target_domain = url.domain().context("Could not extract domain from URL")?;

    // First navigate to the domain's root page to ensure we can set cookies
    let root_url = format!("{}://{}", url.scheme(), target_domain);
    client.goto(&root_url).await?;

    let file = File::open(cookie_file)
        .with_context(|| format!("Failed to open cookie file: {}", cookie_file))?;

    let reader = BufReader::new(file);
    let cookies: Vec<Cookie> =
        serde_json::from_reader(reader).context("Failed to parse cookie file")?;

    let domain_cookies: Vec<Cookie> = cookies
        .into_iter()
        .filter(|cookie| {
            cookie.domain.as_ref().map_or(false, |cookie_domain| {
                target_domain.ends_with(cookie_domain) || cookie_domain.ends_with(target_domain)
            })
        })
        .collect();

    if domain_cookies.is_empty() {
        warn!("No cookies found for domain: {}", target_domain);
        return Ok(false);
    }

    let mut cookies_added = false;
    for cookie in domain_cookies {
        // Create a Fantoccini cookie with all available properties
        let mut fantoccini_cookie =
            FantocciniCookie::build((cookie.name.clone(), cookie.value.clone()));

        // Set optional properties if available
        if let Some(domain) = cookie.domain {
            fantoccini_cookie = fantoccini_cookie.domain(domain);
        }
        if let Some(path) = cookie.path {
            fantoccini_cookie = fantoccini_cookie.path(path);
        }
        if let Some(secure) = cookie.secure {
            fantoccini_cookie = fantoccini_cookie.secure(secure);
        }
        if let Some(http_only) = cookie.http_only {
            fantoccini_cookie = fantoccini_cookie.http_only(http_only);
        }

        client.add_cookie(fantoccini_cookie.into()).await?;
        cookies_added = true;
    }

    Ok(cookies_added)
}

pub async fn check_domain_cookies(
    client: &mut Client,
    target_url: &str,
    cookie_file: &str,
) -> Result<bool> {
    let url = Url::parse(target_url)?;
    let domain = url.domain().unwrap_or("");

    // Read expected cookies from file
    let file = File::open(cookie_file)
        .with_context(|| format!("Failed to open cookie file: {}", cookie_file))?;
    let reader = BufReader::new(file);
    let cookies: Vec<Cookie> =
        serde_json::from_reader(reader).context("Failed to parse cookie file")?;

    // Filter cookies for this domain
    let expected_cookies: Vec<Cookie> = cookies
        .into_iter()
        .filter(|cookie| {
            cookie.domain.as_ref().map_or(false, |cookie_domain| {
                domain.ends_with(cookie_domain) || cookie_domain.ends_with(domain)
            })
        })
        .collect();

    if expected_cookies.is_empty() {
        info!("No cookies expected for domain: {}", domain);
        return Ok(true);
    }

    // Get current browser cookies
    let current_cookies = client.get_all_cookies().await?;

    // Check if all expected cookies exist
    let mut all_cookies_match = true;
    for expected in &expected_cookies {
        let found = current_cookies.iter().any(|current| {
            // Match by name and domain
            let name_matches = current.name() == expected.name;
            let domain_matches = current.domain().map_or(false, |current_domain| {
                domain.ends_with(current_domain) || current_domain.ends_with(domain)
            });

            name_matches && domain_matches
        });

        if !found {
            warn!("Missing cookie for domain {}: {}", domain, expected.name);
            all_cookies_match = false;
            break;
        }
    }

    info!(
        "Domain {} has all expected cookies: {}",
        domain,
        if all_cookies_match { "Yes" } else { "No" }
    );

    Ok(all_cookies_match)
}

pub async fn manage_domain_cookies(
    client: &mut Client,
    target_url: &str,
    cookie_file: &str,
) -> Result<bool> {
    // Check if cookies exist for this domain
    let cookies_exist = check_domain_cookies(client, target_url, cookie_file).await?;

    // If cookies don't exist, load them
    if !cookies_exist {
        let loaded = load_cookies_for_domain(client, cookie_file, target_url).await?;

        if loaded {
            // Retry navigation after loading cookies
            client.goto(target_url).await?;
        }

        return Ok(loaded);
    }

    Ok(true)
}
