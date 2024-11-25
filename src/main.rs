use anyhow::Result;
use clap::Parser;
use fantoccini::ClientBuilder;
use log::info;

use web_automator::WebAutomator;

/// Web Automator - A flexible web automation tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(value_name = "config")]
    config: String,

    /// Load cookies from a JSON file
    #[arg(long, value_name = "file")]
    cookies: Option<String>,

    /// Custom Selenium WebDriver URL
    #[arg(long, value_name = "url", default_value = "http://localhost:4444")]
    webdriver: String,

    /// Specify browser (firefox or chrome)
    #[arg(long, value_name = "browser", default_value = "firefox")]
    browser: String,

    /// Process multiple links from a file
    #[arg(long, value_name = "file")]
    links: Option<String>,

    /// Enable random order of links
    #[arg(long, default_value = "true")]
    random_order: bool,

    /// Enable headless mode
    #[arg(long)]
    headless: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Parse command line arguments
    let args = Args::parse();

    info!("Connecting to WebDriver at {}", args.webdriver);

    // Set up browser capabilities
    let mut capabilities = serde_json::json!({
        "setWindowRect": true,
        "moz:firefoxOptions": {
            "prefs": {
                "intl.accept_languages": "en-GB"
            },
            "args": [
                if args.headless { "--headless" } else { "" },
                "--enable-automation=False",
                "--disable-blink-features=AutomationControlled"
            ]
        },
        "timeouts": {
            "pageLoad": 10_000,
            "implicit": 5_000,
            "script": 120_000,
        }
    })
    .as_object()
    .unwrap()
    .to_owned();
    match args.browser.as_str() {
        "firefox" => {
            capabilities.insert(
                "browserName".to_string(),
                serde_json::Value::String("firefox".to_string()),
            );
        }
        "chrome" => {
            capabilities.insert(
                "browserName".to_string(),
                serde_json::Value::String("chrome".to_string()),
            );
        }
        other => {
            return Err(anyhow::anyhow!(
                "Unsupported browser: {}. Use 'firefox' or 'chrome'",
                other
            ));
        }
    }

    // Create WebDriver client
    let client = ClientBuilder::native()
        .capabilities(capabilities)
        .connect(&args.webdriver)
        .await?;
    client.set_window_size(1024, 3840).await?;

    // Create WebAutomator instance
    let mut automator = WebAutomator::new(
        client,
        args.cookies,
        &args.config,
        args.links.as_deref(),
        args.random_order,
    );

    // Run automation
    automator.run_automation().await?;

    Ok(())
}
