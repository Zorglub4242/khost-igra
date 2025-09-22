// Service management for IGRA Orchestra

use std::path::Path;
use std::process::Command;
use crate::igra::ServiceStatus;

pub fn check_all_services(igra_dir: &Path) -> Result<Vec<ServiceStatus>, String> {
    let output = Command::new("docker")
        .args(&["compose", "ps", "--format", "json"])
        .current_dir(igra_dir)
        .output()
        .map_err(|e| format!("Failed to get service status: {}", e))?;

    let json_str = String::from_utf8_lossy(&output.stdout);
    let mut services = Vec::new();

    // Parse JSON manually (simplified for this implementation)
    for line in json_str.lines() {
        if line.contains("execution-layer") {
            services.push(ServiceStatus {
                name: "execution-layer".to_string(),
                running: line.contains("running"),
                status: extract_status(line),
            });
        } else if line.contains("block-builder") {
            services.push(ServiceStatus {
                name: "block-builder".to_string(),
                running: line.contains("running"),
                status: extract_status(line),
            });
        } else if line.contains("viaduct") {
            services.push(ServiceStatus {
                name: "viaduct".to_string(),
                running: line.contains("running"),
                status: extract_status(line),
            });
        }
    }

    if services.is_empty() {
        // Fallback to checking individual services
        services = check_services_individually()?;
    }

    Ok(services)
}

fn check_services_individually() -> Result<Vec<ServiceStatus>, String> {
    let service_names = vec!["execution-layer", "block-builder", "viaduct"];
    let mut services = Vec::new();

    for name in service_names {
        let output = Command::new("docker")
            .args(&["ps", "-q", "-f", &format!("name={}", name)])
            .output()
            .map_err(|e| format!("Failed to check {}: {}", name, e))?;

        let running = !output.stdout.is_empty();
        services.push(ServiceStatus {
            name: name.to_string(),
            running,
            status: if running { "running".to_string() } else { "stopped".to_string() },
        });
    }

    Ok(services)
}

fn extract_status(json_line: &str) -> String {
    if json_line.contains("running") {
        "running".to_string()
    } else if json_line.contains("exited") {
        "exited".to_string()
    } else {
        "unknown".to_string()
    }
}

pub fn restart_service(service_name: &str) -> Result<(), String> {
    let output = Command::new("docker")
        .args(&["restart", service_name])
        .output()
        .map_err(|e| format!("Failed to restart {}: {}", service_name, e))?;

    if !output.status.success() {
        return Err(format!("Failed to restart {}: {}",
            service_name, String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}