// Standard library imports
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// External crate imports
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_yaml;
use url::Url;

// Local module imports
use crate::models::Instruction;

#[derive(Debug, Deserialize)]
pub struct ConfigYaml {
    pub domains: HashMap<String, DomainConfig>,
}

#[derive(Debug, Deserialize)]
pub struct DomainConfig {
    /// Path to the instruction file for this domain
    pub instructions: String,

    /// Optional selectors to skip if found
    #[serde(default)]
    pub skip_elements: Option<Vec<String>>,

    /// Optional loop configurations
    #[serde(default)]
    pub loop_config: Option<Vec<LoopConfig>>,

    /// Optional cookie file path for this domain
    #[serde(default)]
    pub cookie_file: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoopConfig {
    /// Number of times to loop
    pub times: u32,
    /// From step
    pub from_action_num: u32,
    /// To step
    pub to_action_num: u32,
}

impl ConfigYaml {
    /// Load configuration from a YAML file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config_content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;

        let config: ConfigYaml =
            serde_yaml::from_str(&config_content).context("Failed to parse configuration file")?;

        Ok(config)
    }

    /// Get configuration for a specific domain
    pub fn get_domain_config(&self, domain: &str) -> Option<&DomainConfig> {
        self.domains.get(domain)
    }
}

/// Find instruction file for a given domain
pub fn find_instruction_file_for_domain(
    config_path: &str,
    domain: &str,
) -> Result<(PathBuf, Option<Vec<String>>)> {
    // Load configuration
    let config = ConfigYaml::load_from_file(config_path)?;

    // Determine the base directory for resolving relative paths
    let base_dir = PathBuf::from(config_path)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
        .to_path_buf();

    // If domain is in config, use the specified instruction file
    if let Some(domain_config) = config.get_domain_config(domain) {
        let instruction_path = PathBuf::from(&domain_config.instructions);

        // If the path is absolute, return it directly
        if instruction_path.is_absolute() {
            return Ok((instruction_path, domain_config.skip_elements.clone()));
        }

        // If relative, resolve from the base directory
        let resolved_path = base_dir.join(&instruction_path);

        // Check if the resolved path exists
        if resolved_path.exists() {
            return Ok((resolved_path, domain_config.skip_elements.clone()));
        }

        // If the specified path doesn't exist, return an error
        return Err(anyhow::anyhow!(
            "Instruction file not found for domain: {}. \
            Attempted path: {}",
            domain,
            resolved_path.display()
        ));
    }

    // If no configuration found for the domain
    Err(anyhow::anyhow!(
        "No configuration found for domain: {} in config file: {}",
        domain,
        config_path
    ))
}

/// Load and parse instruction file
pub fn load_instructions_file(path: &Path) -> Result<Vec<Instruction>> {
    let contents = fs::read_to_string(path).context("Failed to read instruction file")?;

    serde_yaml::from_str(&contents).context("Failed to parse instruction file")
}

/// Convenience function to get domain from a URL
pub fn get_domain_from_url(url: &str) -> Result<String> {
    let parsed_url = Url::parse(url)?;
    parsed_url
        .domain()
        .map(|d| d.to_string())
        .ok_or_else(|| anyhow::anyhow!("Invalid domain"))
}
