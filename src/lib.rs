pub mod config;
pub mod cookie_manager;
pub mod instruction_handler;
pub mod link_parser;
pub mod models;
pub mod timing_utils;

use anyhow::Result;
use config::LoopConfig;
use fantoccini::{Client, Locator};
use log::{info, warn};
use rand::prelude::SliceRandom;

pub struct WebAutomator {
    client: Client,
    cookie_file: Option<String>,
    config: config::ConfigYaml,
    config_path: Option<String>,
    links_file: Option<String>,
    random_order: bool,
}

impl WebAutomator {
    pub fn new(
        client: Client,
        cookie_file: Option<String>,
        config_file: &str,
        links_file: Option<&str>,
        random_order: bool,
    ) -> Self {
        let config = config::ConfigYaml::load_from_file(config_file).unwrap();
        Self {
            client,
            cookie_file,
            config,
            config_path: Some(config_file.to_string()),
            links_file: links_file.map(|s| s.to_string()),
            random_order,
        }
    }

    pub async fn run_automation(&mut self) -> Result<()> {
        // If links file is provided, process multiple links
        if let Some(links_file) = &self.links_file {
            let mut links = link_parser::parse_links_file(links_file)?;
            if self.random_order {
                links.shuffle(&mut rand::thread_rng());
            }
            for link in links {
                let (url, _) = link;
                {
                    let domain = config::get_domain_from_url(&url)?;

                    // Execute instruction for each link
                    self.execute_instruction(&url, &domain).await?;
                }
            }
        }

        Ok(())
    }

    pub async fn execute_instruction(&mut self, url: &str, domain: &str) -> Result<()> {
        // Find instruction file for the domain
        let res =
            config::find_instruction_file_for_domain(self.config_path.as_deref().unwrap(), domain);
        if res.is_err() {
            warn!("Could not find instruction file for domain: {}", domain);
            return Ok(());
        }
        let (instruction_file, skip_elements) = res?;

        // Check if any skip elements exist
        let should_skip = if let Some(elements) = skip_elements {
            self.check_skip_elements(&elements).await
        } else {
            false
        };

        // Skip this link if skip elements are found
        if should_skip {
            info!("Skipping link due to presence of skip elements: {}", url);
            return Ok(());
        }

        // Navigate to the URL first
        self.client.goto(url).await?;

        // Check and load domain-specific cookies
        let cookie_file = self.cookie_file.clone();
        if let Some(cookie_path) = cookie_file {
            // Use the new manage_domain_cookies function
            cookie_manager::manage_domain_cookies(&mut self.client, url, &cookie_path).await?;
        }

        // Load loop configuration
        let loop_configs = self
            .config
            .get_domain_config(domain)
            .and_then(|config| config.loop_config.clone());

        // Load and parse instructions
        let instructions = config::load_instructions_file(&instruction_file)?;

        let mut instruction_index = 0;
        while instruction_index < instructions.len() {
            // Check if current instruction index is the start of a loop
            if let Some(loop_config) =
                loop_configs.as_ref().and_then(|configs: &Vec<LoopConfig>| {
                    configs
                        .iter()
                        .find(|cfg| cfg.from_action_num as usize == instruction_index)
                })
            {
                // Execute the loop
                for _ in 0..loop_config.times {
                    for i in loop_config.from_action_num..=loop_config.to_action_num {
                        if let Some(instruction) = instructions.get(i as usize) {
                            instruction_handler::handle_instruction(&mut self.client, instruction)
                                .await?;
                        }
                    }
                }
                // Move the instruction index to after the loop
                instruction_index = loop_config.to_action_num as usize + 1;
            } else {
                // Execute instruction normally
                if let Some(instruction) = instructions.get(instruction_index) {
                    instruction_handler::handle_instruction(&mut self.client, instruction).await?;
                }
                instruction_index += 1;
            }
        }

        Ok(())
    }

    async fn check_skip_elements(&mut self, skip_selectors: &[String]) -> bool {
        for selector in skip_selectors {
            match self.client.find(Locator::Css(selector)).await {
                Ok(_) => {
                    warn!("Found skip element: {}", selector);
                    return true;
                }
                Err(_) => continue,
            }
        }
        false
    }
}
