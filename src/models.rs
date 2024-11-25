use serde::Deserialize;

/// Represents a browser automation instruction
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "action")]
pub enum Instruction {
    /// Navigate to a URL
    #[serde(rename = "navigate")]
    Navigate {
        /// Target URL
        url: String,
        /// If true, navigation failure will stop execution
        critical: Option<bool>,
    },

    /// Click an element
    #[serde(rename = "click")]
    Click {
        /// Element selector
        selector: String,
        /// Selector type: "css", "id", or "xpath"
        by: Option<String>,
        /// Click timeout in seconds
        timeout: Option<u64>,
        /// Optional delay between clicks in seconds
        delay: Option<f64>,
        /// Optional standard deviation for delay
        delay_stdev: Option<f64>,
        /// Optional: ignore if element is not found
        ignore_errors: Option<bool>,
    },

    /// Wait for a specified duration, optionally with randomized variance
    #[serde(rename = "wait")]
    Wait {
        /// Base wait time in seconds
        seconds: f64,
        /// Optional standard deviation for randomized wait time
        /// If specified, the actual wait time will be drawn from a normal distribution
        /// centered around `seconds` with the given standard deviation
        stdev: Option<f64>,
    },

    /// Scroll the page
    #[serde(rename = "scroll")]
    Scroll {
        /// Scroll amount in pixels
        amount: Option<i64>,
    },

    /// Click a random element matching a selector
    #[serde(rename = "random_click")]
    RandomClick {
        /// Element selector to find multiple matches
        selector: String,
        /// Selector type: "css", "id", or "xpath"
        by: Option<String>,
        /// Optional list of text substrings to exclude
        exclude_text: Option<Vec<String>>,
        /// Optional timeout in seconds
        timeout: Option<u64>,
        /// Optional exhaustive mode to click all elements
        exhaustive: Option<bool>,
        /// Optional delay between clicks in seconds
        delay: Option<f64>,
        /// Optional standard deviation for delay
        delay_stdev: Option<f64>,
    },
}

/// Represents a browser cookie
#[derive(Debug, Deserialize)]
pub struct Cookie {
    /// Cookie name
    pub name: String,
    /// Cookie value
    pub value: String,
    /// Cookie domain
    pub domain: Option<String>,
    /// Cookie path
    pub path: Option<String>,
    /// Whether the cookie is secure
    pub secure: Option<bool>,
    /// Whether the cookie is HTTP only
    pub http_only: Option<bool>,
    /// Cookie expiration timestamp in seconds since epoch
    pub expiry: Option<u64>,
}
