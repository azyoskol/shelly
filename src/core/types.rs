// src/core/types.rs
// Core type definitions for the Shally framework

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// A simplified representation of shell environment variables
pub type ShellContext = HashMap<String, String>;

/// Configuration structure loaded from YAML file or defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    #[serde(default = "default_shell_name")]
    pub shell_name: String,
    #[serde(default)]
    pub settings: HashMap<String, String>,
}

fn default_shell_name() -> String {
    "zsh".to_string()
}

impl Default for PluginConfig {
    fn default() -> Self {
        PluginConfig {
            shell_name: default_shell_name(),
            settings: HashMap::new(),
        }
    }
}
