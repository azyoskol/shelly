// src/config/mod.rs
// Configuration loading and resolution module

use std::fs;
use std::path::{Path, PathBuf};
use crate::core::PluginConfig;
use crate::errors::{ShallyError, ShallyResult};

const DEFAULT_CONFIG_FILENAME: &str = "shally.yaml";

/// Attempts to load PluginConfig from a YAML file at the given path.
/// Returns Default config if file doesn't exist or can't be parsed.
pub fn load_config(path: &Path) -> PluginConfig {
    load_config_from_path(path).unwrap_or_default()
}

/// Internal helper that returns Option so callers can distinguish "not found" from "bad file".
pub fn load_config_from_path(path: &Path) -> Option<PluginConfig> {
    let contents = fs::read_to_string(path).ok()?;
    let config: PluginConfig = serde_yaml::from_str(&contents).ok()?;
    Some(config)
}

/// Load configuration with proper error handling
pub fn load_config_with_error(path: &Path) -> ShallyResult<PluginConfig> {
    let contents = fs::read_to_string(path)
        .map_err(|_e| ShallyError::ConfigNotFound(path.display().to_string()))?;
    
    serde_yaml::from_str(&contents)
        .map_err(|e| ShallyError::ConfigParseError(e.to_string()))
}

/// Resolves the config file path from CLI argument, environment variable, or default locations.
/// Returns None if no config file is found anywhere.
pub fn resolve_config_path(explicit_path: Option<&str>) -> Option<PathBuf> {
    // 1. Explicit --config path
    if let Some(p) = explicit_path {
        return Some(PathBuf::from(p));
    }

    // 2. SHALLY_CONFIG environment variable
    if let Ok(env_path) = std::env::var("SHALLY_CONFIG") {
        let p = PathBuf::from(&env_path);
        if p.exists() {
            return Some(p);
        }
    }

    // 3. Check default locations: current dir, then home dir
    let candidates = [
        PathBuf::from(DEFAULT_CONFIG_FILENAME),
        dirs_home(),
    ];

    for candidate in &candidates {
        if candidate.exists() {
            return Some(candidate.clone());
        }
    }

    None
}

/// Returns the path to ~/.shally.yaml, or None if home dir can't be determined.
fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(|h| PathBuf::from(h).join(DEFAULT_CONFIG_FILENAME))
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_CONFIG_FILENAME))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

    #[test]
    fn test_load_valid_yaml() {
        let yaml = r#"
shell_name: fish
settings:
  api_endpoint: "https://api.example.com"
  timeout: "30"
"#;
        let config: PluginConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.shell_name, "fish");
        assert_eq!(config.settings.get("api_endpoint").unwrap(), "https://api.example.com");
        assert_eq!(config.settings.get("timeout").unwrap(), "30");
    }

    #[test]
    fn test_load_minimal_yaml() {
        let yaml = r#"
shell_name: zsh
"#;
        let config: PluginConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.shell_name, "zsh");
        assert!(config.settings.is_empty());
    }

    #[test]
    fn test_load_empty_settings() {
        let yaml = r#"
shell_name: fish
settings: {}
"#;
        let config: PluginConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.shell_name, "fish");
        assert!(config.settings.is_empty());
    }

    #[test]
    fn test_default_shell_name() {
        let yaml = r#"
settings:
  key: "value"
"#;
        let config: PluginConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.shell_name, "zsh");
        assert_eq!(config.settings.get("key").unwrap(), "value");
    }

    #[test]
    fn test_full_default() {
        let yaml = r#"{}"#;
        let config: PluginConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.shell_name, "zsh");
        assert!(config.settings.is_empty());
    }

    #[test]
    fn test_invalid_yaml_returns_none() {
        let yaml = "this is not: valid: yaml: [";
        let result: Result<PluginConfig, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_config_from_nonexistent_path() {
        let path = Path::new("/tmp/shally_nonexistent_test.yaml");
        let config = load_config(path);
        assert_eq!(config.shell_name, "zsh");
        assert!(config.settings.is_empty());
    }

    #[test]
    fn test_resolve_config_path_explicit() {
        let path = resolve_config_path(Some("/tmp/explicit.yaml"));
        assert!(path.is_some());
        assert_eq!(path.unwrap().file_name().unwrap(), "explicit.yaml");
    }
}
