// Monitoring functions for IGRA services

use std::path::Path;
use std::process::Command;

pub fn get_current_block_height(igra_dir: &Path) -> Result<u64, String> {
    // Check block-builder logs for latest block height
    let output = Command::new("docker")
        .args(&["logs", "block-builder", "--tail", "10"])
        .output()
        .map_err(|e| format!("Failed to get block height: {}", e))?;

    let logs = String::from_utf8_lossy(&output.stdout);

    // Parse block height from logs
    for line in logs.lines().rev() {
        if line.contains("Built block") {
            // Extract block number from log line
            if let Some(block_str) = extract_block_number(line) {
                if let Ok(height) = block_str.parse() {
                    return Ok(height);
                }
            }
        }
    }

    Err("No block height found in logs".into())
}

pub fn check_sync_status(igra_dir: &Path) -> Result<f64, String> {
    // Check viaduct sync status
    let output = Command::new("docker")
        .args(&["logs", "viaduct", "--tail", "20"])
        .output()
        .map_err(|e| format!("Failed to get sync status: {}", e))?;

    let logs = String::from_utf8_lossy(&output.stdout);

    // Parse sync percentage from logs
    for line in logs.lines().rev() {
        if line.contains("sync") || line.contains("Sync") {
            // Extract sync percentage
            if let Some(percent) = extract_sync_percentage(line) {
                return Ok(percent);
            }
        }
    }

    // If no sync info found, assume 100%
    Ok(100.0)
}

pub fn check_block_production_rate() -> Result<f32, String> {
    // Check blocks produced in last 5 minutes
    let output = Command::new("bash")
        .args(&["-c", "docker logs block-builder --since 5m 2>&1 | grep -c 'Built block'"])
        .output()
        .map_err(|e| format!("Failed to get block rate: {}", e))?;

    let count_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let block_count: f32 = count_str.parse().unwrap_or(0.0);

    // Return blocks per minute
    Ok(block_count / 5.0)
}

pub fn get_service_health() -> Result<Vec<(String, bool)>, String> {
    let services = vec![
        "execution-layer",
        "block-builder",
        "viaduct",
    ];

    let mut health_status = Vec::new();

    for service in services {
        let output = Command::new("docker")
            .args(&["ps", "-q", "-f", &format!("name={}", service)])
            .output()
            .map_err(|e| format!("Failed to check service {}: {}", service, e))?;

        let is_running = !output.stdout.is_empty();
        health_status.push((service.to_string(), is_running));
    }

    Ok(health_status)
}

fn extract_block_number(log_line: &str) -> Option<String> {
    // Extract block number from a log line like "Built block #12345"
    if let Some(pos) = log_line.find("#") {
        let after_hash = &log_line[pos + 1..];
        let number: String = after_hash
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();

        if !number.is_empty() {
            return Some(number);
        }
    }
    None
}

fn extract_sync_percentage(log_line: &str) -> Option<f64> {
    // Extract percentage from log lines containing sync status
    // Looking for patterns like "99.5%" or "sync: 99.5"
    for word in log_line.split_whitespace() {
        if word.ends_with('%') {
            if let Ok(percent) = word[..word.len()-1].parse() {
                return Some(percent);
            }
        }
    }
    None
}