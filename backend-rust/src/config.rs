/**
 * Configuration Pyramid:
 * L1: Core environment variables
 * L2: Default configurations
 * L3: Derived settings
 * L4: Configuration validation
 */

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_repo_url")]
    pub repo_url: String,
    
    #[serde(default = "default_cache_dir")]
    pub cache_dir: String,
    
    #[serde(default = "default_mongodb_uri")]
    pub mongodb_uri: String,
    
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Config {
    pub fn validate(&self) -> Result<(), String> {
        if !PathBuf::from(&self.cache_dir).exists() {
            std::fs::create_dir_all(&self.cache_dir)
                .map_err(|e| format!("Failed to create cache dir: {}", e))?;
        }
        Ok(())
    }

    pub fn from_env() -> Self {
        envy::from_env::<Config>().unwrap_or_default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            repo_url: default_repo_url(),
            cache_dir: default_cache_dir(),
            mongodb_uri: default_mongodb_uri(),
            port: default_port(),
        }
    }
}

fn default_repo_url() -> String {
    std::env::var("REPO_URL")
        .unwrap_or_else(|_| "https://github.com/facebook/react.git".to_string())
}

fn default_cache_dir() -> String {
    std::env::var("CACHE_DIR")
        .unwrap_or_else(|_| "/tmp/parseltongue-cache".to_string())
}

fn default_mongodb_uri() -> String {
    std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string())
}

fn default_port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8081)
} 