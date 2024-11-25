use anyhow::Result;
use rand::Rng;
use rand_distr::Normal;

/// Generates a random wait time based on a normal distribution
///
/// # Arguments
///
/// * `mean` - The mean of the normal distribution
/// * `stdev` - The standard deviation of the normal distribution
///
/// # Returns
///
/// A random wait time, ensuring it is non-negative
pub fn random_wait_time(mean: f64, stdev: f64) -> Result<f64> {
    let normal = Normal::new(mean, stdev)
        .map_err(|_| anyhow::anyhow!("Invalid normal distribution parameters"))?;

    // Draw a random wait time from the distribution
    let mut rng = rand::thread_rng();
    let random_wait = rng.sample(normal);

    // Ensure wait time is non-negative
    Ok(random_wait.max(0.0))
}
