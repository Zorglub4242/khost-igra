// Docker Compose management for IGRA Orchestra

use std::path::Path;
use std::process::{Command, Output};

pub fn compose_up(igra_dir: &Path, profile: &str) -> Result<(), String> {
    let output = Command::new("docker")
        .args(&["compose", "--profile", profile, "up", "-d"])
        .current_dir(igra_dir)
        .output()
        .map_err(|e| format!("Failed to start services: {}", e))?;

    if !output.status.success() {
        return Err(format!("Docker compose up failed: {}",
            String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

pub fn compose_down(igra_dir: &Path) -> Result<(), String> {
    let output = Command::new("docker")
        .args(&["compose", "down"])
        .current_dir(igra_dir)
        .output()
        .map_err(|e| format!("Failed to stop services: {}", e))?;

    if !output.status.success() {
        return Err(format!("Docker compose down failed: {}",
            String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

pub fn compose_ps(igra_dir: &Path) -> Result<String, String> {
    let output = Command::new("docker")
        .args(&["compose", "ps", "--format", "json"])
        .current_dir(igra_dir)
        .output()
        .map_err(|e| format!("Failed to get service status: {}", e))?;

    Ok(String::from_utf8_lossy(&output.stdout).into())
}

pub fn docker_logs(service: &str, lines: u32) -> Result<String, String> {
    let output = Command::new("docker")
        .args(&["logs", service, "--tail", &lines.to_string()])
        .output()
        .map_err(|e| format!("Failed to get logs: {}", e))?;

    Ok(String::from_utf8_lossy(&output.stdout).into())
}