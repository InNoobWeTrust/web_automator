---
runme:
  id: 01JDJ6ET311MNGPD64V44QJWG4
  version: v3
---

# Web Automator

A flexible Rust-based web automation tool using Fantoccini for browser automation.

Made with the help of AI agent from [Windsurf](https://codeium.com/windsurf)

## Features

- Automated web browsing using Selenium WebDriver
- YAML-based instruction set
- Support for multiple browsers (Firefox, Chrome)
- Cookie management and persistence
- Screenshot capabilities
- Links file support for batch processing
- Configurable timeouts and error handling
- Detailed logging with different verbosity levels

## Prerequisites

- Rust (latest stable version)
- Selenium WebDriver server running
- WebDriver-compatible browser (Firefox or Chrome)

## Dependencies

```toml {"id":"01JDJTTX50730MNSKNXYR56HTN"}
anyhow = "1.0"
clap = { version = "4.0", features = ["derive"] }
log = "0.4"
tokio = { version = "1", features = ["full"] }
url = "2.2"
rand = "0.8"
rand_distr = "0.4"
fantoccini = "0.21.2"
env_logger = "0.11.5"
serde = "1.0.215"
serde_json = "1.0.133"
serde_yaml = "0.9.34"
tokio-stream = "0.1.16"
```

## Project Structure

```ini {"id":"01JDJTTX51P68BWYJ9PMEME2RA"}
web_automator/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Core automation logic
│   ├── models.rs            # Data structures and types
│   ├── config.rs            # Configuration handling
│   ├── cookie_manager.rs    # Cookie handling
│   ├── instruction_handler.rs# Instruction processing
│   ├── link_parser.rs       # Link file parsing
│   └── timing_utils.rs      # Timing and delay utilities
├── Cargo.toml               # Project configuration
└── README.md               # This file
```

## Installation

1. Clone the repository
2. Ensure Selenium WebDriver is running
3. Install dependencies:

```bash {"id":"01JDJTTX51P68BWYJ9PPRXAM9J"}
cargo build
```

## Usage

### Basic Usage

Run the automator with a YAML config file:

```bash {"id":"01JDJTTX51P68BWYJ9PQZ7C294"}
cargo run -- config.yaml
```

### Command Line Options

```bash {"id":"01JDJTTX51P68BWYJ9PTKV5F0E"}
cargo run -- <config_file> [options]

Arguments:
  <config_file>             Path to YAML config file

Options:
  --cookies <file>          Load cookies from a JSON file
  --webdriver <url>         Custom Selenium WebDriver URL [default: http://localhost:4444]
  --browser <browser>       Specify browser (firefox or chrome) [default: firefox]
  --links <file>           Process multiple links from a file
  --random-order           Enable random order of links [default: true]
  --headless               Enable headless mode
  -h, --help               Print help information
  -V, --version            Print version information
```

### Example Commands

```bash {"id":"01JDJTTX51P68BWYJ9PV7A43XB"}
# Basic usage with config file
cargo run -- config.yaml

# Use Chrome browser in headless mode
cargo run -- config.yaml --browser chrome --headless

# Custom WebDriver URL and cookies
cargo run -- config.yaml --webdriver http://localhost:9515 --cookies cookies.json

# Process multiple links in random order
cargo run -- config.yaml --links links.txt --random-order
```

### Links File Format

The links file should contain one URL per line with optional comments:

```text {"id":"01JDJTTX51P68BWYJ9PVXEYHSE"}
# This is a comment line
https://www.example.com/page1  # Optional description
https://www.example.com/page2
```

### YAML Instructions Format

Create a YAML file with supported instructions:

```yaml {"id":"01JDJTTX51P68BWYJ9PWQJ46VX"}
# Navigate to a URL
- action: navigate
  url: "https://www.example.com"
  critical: true  # Optional, defaults to false

# Click an element
- action: click
  selector: "#login-button"
  by: "css"        # Optional: css, id, or xpath (default: css)
  timeout: 10      # Optional: timeout in seconds
  delay: 1         # Optional: delay after clicking
  delay_stdev: 0.2 # Optional: random variation in delay
  ignore_errors: false # Optional: continue if element not found

# Wait with random variation
- action: wait
  seconds: 2
  stdev: 0.5  # Optional: adds random variation to wait time

# Scroll the page
- action: scroll
  amount: 500  # Optional: scroll amount in pixels

# Click random elements
- action: random_click
  selector: ".like-button"
  by: "css"        # Optional: css, id, or xpath
  exclude_text:    # Optional: skip elements containing these texts
    - "Follow"
    - "Save post"
    - "Save link"
  timeout: 10      # Optional: timeout in seconds
  exhaustive: true # Optional: click all matching elements
  delay: 5         # Optional: delay between clicks
  delay_stdev: 0.5 # Optional: random variation in delay
```

### Cookie File Format

The cookie file should be a JSON array of cookie objects:

```json {"id":"01JDJTTX51P68BWYJ9PZ3V7T66"}
[
  {
    "name": "sessionId",
    "value": "abc123",
    "domain": "example.com",
    "path": "/",
    "secure": true,
    "http_only": true,
    "expiry": 1735689600
  },
  {
    "name": "theme",
    "value": "dark",
    "domain": "example.com",
    "path": "/"
  }
]
```

Cookie fields:

- `name`: Cookie name (required)
- `value`: Cookie value (required)
- `domain`: Cookie domain (optional)
- `path`: Cookie path (optional)
- `secure`: Whether the cookie is secure (optional)
- `http_only`: Whether the cookie is HTTP only (optional)
- `expiry`: Cookie expiration timestamp in seconds since epoch (optional)

## Configuration

### Environment Variables

- `RUST_LOG`: Set logging level (error, warn, info, debug, trace)

```bash {"id":"01JDJTTX51P68BWYJ9Q26KP3VR"}
RUST_LOG=debug cargo run
```

- `SELENIUM_URL`: Override default Selenium WebDriver URL
- `DEFAULT_BROWSER`: Set default browser (firefox/chrome)

### Configuration File Format

The configuration file (config.yml) defines domain-specific settings and instruction sets:

```yaml {"id":"01JDJTTX51P68BWYJ9Q9NWQKTR"}
domains:
  www.example.com:
    # Path to domain-specific instruction file
    instructions: example_instructions.yml
    
    # Optional: Elements to skip during automation
    skip_elements:
      - "div.popup"
      - "button.notification"
    
    # Optional: Loop specific instruction sequences
    loop_config:
      - times: 3              # Number of times to loop
        from_action_num: 8    # Start from instruction #8
        to_action_num: 12     # End at instruction #12
    
    # Optional: Domain-specific cookie file
    cookie_file: example_cookies.json

  www.another-site.com:
    instructions: another_instructions.yml
    skip_elements:
      - "div.private-content"
      - "span.age-restricted"
```

The configuration file supports:
- Multiple domains with different instruction sets
- Skip elements: CSS selectors for elements to ignore
- Loop configurations: Repeat specific sequences of instructions
- Cookie files: Domain-specific cookie configurations

Each domain's instruction file (`instructions` field) contains the sequence of actions to perform. The path can be absolute or relative to the config file's location.

## Development

### Getting Started

1. Install development dependencies:

```bash {"id":"01JDJTTX51P68BWYJ9Q3SFB9CD"}
make dev-deps
```

### Available Commands

The project uses a Makefile to organize common development tasks:

```bash {"id":"01JDJTTX51P68BWYJ9Q45RRGM0"}
make help         # Show all available commands
make build        # Build the project
make release      # Build for release
make check        # Run all checks (format, lint, test)
make test         # Run tests
make lint         # Run linter (clippy)
make format       # Format code
make format-check # Check code formatting
make clean        # Clean build artifacts
make run          # Run the project
```

### Code Style and Linting

The project enforces code style and quality through:

- `rustfmt` for consistent code formatting
- `clippy` for catching common mistakes and enforcing best practices

Run all checks before submitting changes:

```bash {"id":"01JDJTTX51P68BWYJ9Q73R98CY"}
make check
```

Or run individual checks:

```bash {"id":"01JDJTTX51P68BWYJ9Q8VAQK5S"}
make format-check  # Check code formatting
make lint         # Run clippy linter
```

## Error Handling

The automator implements robust error handling:

- Retries for flaky operations
- Detailed error messages
- Critical vs non-critical instruction handling
- Timeout configuration for each operation

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit changes
4. Push to the branch
5. Create a Pull Request

## License

MIT License

## Troubleshooting

Common issues and solutions:

- Ensure Selenium WebDriver is running and accessible
- Check browser compatibility and WebDriver versions
- Verify YAML instruction syntax
- Use debug logging for detailed operation information
- Check network connectivity for remote operations
