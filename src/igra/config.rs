// Configuration management for IGRA Orchestra

use std::fs;
use std::path::Path;
use std::collections::HashMap;

pub struct IgraConfig {
    pub node_name: String,
    pub health_api_key: Option<String>,
    pub kaspad_host: String,
    pub kaspad_port: u16,
    pub network: String,
}

impl Default for IgraConfig {
    fn default() -> Self {
        Self {
            node_name: "merlin".to_string(),
            health_api_key: None,
            kaspad_host: "host.docker.internal".to_string(),
            kaspad_port: 17110,
            network: "testnet".to_string(),
        }
    }
}

impl IgraConfig {
    pub fn load_from_env(env_path: &Path) -> Result<Self, String> {
        let content = fs::read_to_string(env_path)
            .map_err(|e| format!("Failed to read .env file: {}", e))?;

        let mut config = Self::default();

        for line in content.lines() {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    "NODE_ID" => config.node_name = value.trim().to_string(),
                    "HEALTH_CHECK_API_KEY" => {
                        let val = value.trim();
                        if !val.is_empty() && val != "<your-api-key>" {
                            config.health_api_key = Some(val.to_string());
                        }
                    },
                    "KASPAD_HOST" => config.kaspad_host = value.trim().to_string(),
                    "KASPAD_BORSH_PORT" => {
                        if let Ok(port) = value.trim().parse() {
                            config.kaspad_port = port;
                        }
                    },
                    "NETWORK" => config.network = value.trim().to_string(),
                    _ => {}
                }
            }
        }

        Ok(config)
    }

    pub fn save_to_env(&self, env_path: &Path) -> Result<(), String> {
        let content = fs::read_to_string(env_path)
            .map_err(|e| format!("Failed to read .env file: {}", e))?;

        let mut lines: Vec<String> = Vec::new();

        for line in content.lines() {
            if line.starts_with("NODE_ID=") {
                lines.push(format!("NODE_ID={}", self.node_name));
            } else if line.starts_with("HEALTH_CHECK_API_KEY=") {
                lines.push(format!("HEALTH_CHECK_API_KEY={}",
                    self.health_api_key.as_ref().unwrap_or(&"".to_string())));
            } else if line.starts_with("KASPAD_HOST=") {
                lines.push(format!("KASPAD_HOST={}", self.kaspad_host));
            } else if line.starts_with("KASPAD_BORSH_PORT=") {
                lines.push(format!("KASPAD_BORSH_PORT={}", self.kaspad_port));
            } else {
                lines.push(line.to_string());
            }
        }

        fs::write(env_path, lines.join("\n"))
            .map_err(|e| format!("Failed to write .env file: {}", e))?;

        Ok(())
    }
}