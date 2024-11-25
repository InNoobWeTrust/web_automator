use anyhow::{Context, Result};
use fantoccini::{Client, Locator};
use log::{error, info};
use rand::Rng;
use std::time::Duration;
use tokio::time::sleep;

use crate::models::Instruction;
use crate::timing_utils::random_wait_time;

pub async fn handle_instruction(client: &mut Client, instruction: &Instruction) -> Result<()> {
    match instruction {
        Instruction::Navigate { url, critical } => {
            info!("Navigating to URL: {}", url);
            let nav_result = client.goto(url).await;

            if critical.unwrap_or(false) && nav_result.is_err() {
                error!("Critical navigation failed: {url}");
                return Err(anyhow::anyhow!("Critical navigation failed: {url}"));
            }

            nav_result.context("Navigation failed")?;
        }

        Instruction::Click {
            selector,
            by,
            timeout,
            delay,
            delay_stdev,
            ignore_errors,
        } => {
            info!("Clicking on element: {}", selector);

            // Determine locator based on 'by' parameter
            let locator = match by.as_deref() {
                Some("id") => Locator::Id(selector),
                Some("xpath") => Locator::XPath(selector),
                _ => Locator::Css(selector),
            };

            // Set default timeout or use provided value
            let wait_timeout = timeout.unwrap_or(10);

            // Use a custom timeout implementation
            let elem =
                tokio::time::timeout(Duration::from_secs(wait_timeout), client.find(locator)).await;

            // Check if the timeout occurred
            if ignore_errors.unwrap_or(false) && elem.is_err() {
                info!("Click ignored due to error: {}", elem.err().unwrap());
                return Ok(());
            }

            let elem = elem.context("Timeout waiting for element")?;

            if ignore_errors.unwrap_or(false) && elem.is_err() {
                info!("Click ignored due to error: {}", elem.err().unwrap());
                return Ok(());
            }

            // Unwrap element
            let elem = elem.context("Failed to find element")?;

            // Click the element by invoking the click method within JS runtime
            client
                .execute("arguments[0].click()", vec![serde_json::to_value(elem)?])
                .await
                .context("Failed to click element")?;
            // elem.click().await.context("Failed to click element")?;

            if let Some(delay) = delay {
                let wait_time = if let Some(delay_stdev) = delay_stdev {
                    random_wait_time(*delay, *delay_stdev)?
                } else {
                    *delay
                };

                info!("Waiting for {} seconds between clicks", wait_time);
                sleep(Duration::from_secs_f64(wait_time)).await;
            }
        }

        Instruction::Wait { seconds, stdev } => {
            // If no standard deviation is specified, use a fixed wait time
            let wait_time = match stdev {
                Some(std_dev) => random_wait_time(*seconds, *std_dev)?,
                None => *seconds,
            };

            info!("Waiting for {} seconds", wait_time);

            // Convert to Duration and wait
            sleep(Duration::from_secs_f64(wait_time)).await;
        }

        Instruction::Scroll { amount } => {
            let scroll_amount = amount.unwrap_or(100);
            let script = format!("window.scrollBy(0, {});", scroll_amount);

            client
                .execute(&script, vec![])
                .await
                .context("Failed to scroll")?;
        }

        Instruction::RandomClick {
            selector,
            by,
            exclude_text,
            timeout,
            exhaustive,
            delay,
            delay_stdev,
        } => {
            info!("Finding random elements to click: {}", selector);

            // Set default timeout or use provided value
            let wait_timeout = timeout.unwrap_or(10);

            // Loop for exhaustive clicking if enabled
            loop {
                // Determine locator based on 'by' parameter
                let locator = match by.as_deref() {
                    Some("id") => Locator::Id(selector),
                    Some("xpath") => Locator::XPath(selector),
                    _ => Locator::Css(selector),
                };

                // Find all matching elements
                let elements = tokio::time::timeout(
                    Duration::from_secs(wait_timeout),
                    client.find_all(locator),
                )
                .await
                .context("Timeout finding elements");

                if elements.is_err() {
                    break;
                }

                let elements = elements?.context("Failed to find elements");

                if elements.is_err() {
                    break;
                }

                let elements = elements?;

                // If no elements found, break loop
                if elements.is_empty() {
                    break;
                }

                // Filter out elements with excluded text
                let filtered_elements = if let Some(exclude_texts) = exclude_text {
                    let mut filtered = Vec::new();
                    for elem in elements {
                        if let Ok(text) = elem.text().await {
                            if !exclude_texts.iter().any(|exclude| text.contains(exclude)) {
                                filtered.push(elem);
                            }
                        } else {
                            filtered.push(elem); // Keep element if text can't be retrieved
                        }
                    }
                    filtered
                } else {
                    elements
                };

                // If no elements remain after filtering, break loop
                if filtered_elements.is_empty() {
                    break;
                }

                // Choose a random element
                let random_index = rand::thread_rng().gen_range(0..filtered_elements.len());
                let elem = filtered_elements[random_index].clone();

                client
                    .execute("arguments[0].click()", vec![serde_json::to_value(&elem)?])
                    .await
                    .context("Failed to click element")?;

                // Add delay if exhaustive is true
                if exhaustive.unwrap_or(false) {
                    if let Some(delay) = delay {
                        let wait_time = if let Some(delay_stdev) = delay_stdev {
                            random_wait_time(*delay, *delay_stdev)?
                        } else {
                            *delay
                        };

                        info!("Waiting for {} seconds between clicks", wait_time);
                        sleep(Duration::from_secs_f64(wait_time)).await;
                    }
                }

                // Break loop if not exhaustive
                if !exhaustive.unwrap_or(false) {
                    break;
                }
            }
        }
    }

    // Small delay between instructions to allow page to process
    sleep(Duration::from_millis(500)).await;

    Ok(())
}
