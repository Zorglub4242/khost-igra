// IGRA L2 Support Module
// Manages IGRA Orchestra services without containing private code

pub mod config;
pub mod docker;
pub mod monitor;
pub mod service;

use std::path::{Path, PathBuf};
use std::process::Command;

pub const IGRA_DIR: &str = "/home/kaspa/igra/igra-orchestra-public";

#[derive(Debug, Clone)]
pub struct IgraManager {
    pub orchestra_path: PathBuf,
    pub node_name: String,
    pub health_api_key: Option<String>,
}

impl IgraManager {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            orchestra_path: PathBuf::from(IGRA_DIR),
            node_name: "merlin".to_string(),
            health_api_key: None,
        })
    }

    pub fn is_installed(&self) -> bool {
        self.orchestra_path.exists()
    }

    pub fn check_services(&self) -> Result<Vec<ServiceStatus>, String> {
        if !self.is_installed() {
            return Err("IGRA Orchestra not found".into());
        }

        service::check_all_services(&self.orchestra_path)
    }

    pub fn start_services(&self) -> Result<(), String> {
        if !self.is_installed() {
            return Err("IGRA Orchestra not found. Run setup first.".into());
        }

        docker::compose_up(&self.orchestra_path, "backend")
    }

    pub fn stop_services(&self) -> Result<(), String> {
        docker::compose_down(&self.orchestra_path)
    }

    pub fn get_block_height(&self) -> Result<u64, String> {
        monitor::get_current_block_height(&self.orchestra_path)
    }
}

#[derive(Debug, Clone)]
pub struct ServiceStatus {
    pub name: String,
    pub running: bool,
    pub status: String,
}