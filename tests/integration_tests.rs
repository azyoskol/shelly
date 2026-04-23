// tests/integration_tests.rs
// Integration tests for Shally Framework

use shally::core::{PluginConfig, ShellContext, ShellPlugin};
use shally::plugin::mock_plugin::MockPlugin;
use shally::config;
use shally::history::HistoryManager;
use std::path::PathBuf;
use tempfile::TempDir;

/// Test plugin installation and hook generation
#[test]
fn test_zsh_install_hook_generation() {
    let binary_path = "shally";
    let snippet = shally::zsh::generate_zshrc_snippet(binary_path);
    
    assert!(snippet.contains("precmd"));
    assert!(snippet.contains("preexec"));
    assert!(snippet.contains("shally"));
    assert!(snippet.contains("--hook"));
}

#[test]
fn test_fish_install_hook_generation() {
    let binary_path = "shally";
    let snippet = shally::fish::generate_config_snippet(binary_path);
    
    assert!(snippet.contains("fish_prompt"));
    assert!(snippet.contains("command_not_found"));
    assert!(snippet.contains("shally"));
    assert!(snippet.contains("--fish-hook"));
}

/// Test configuration loading
#[test]
fn test_config_load_default() {
    let config = PluginConfig::default();
    assert_eq!(config.shell_name, "zsh");
    assert!(config.settings.is_empty());
}

#[test]
fn test_config_load_from_yaml() {
    use std::fs;
    use tempfile::NamedTempFile;
    
    let mut temp_file = NamedTempFile::new().unwrap();
    let yaml_content = r#"
shell_name: fish
settings:
  api_endpoint: "https://api.test.com"
  timeout: "60"
"#;
    
    fs::write(&temp_file, yaml_content).unwrap();
    
    let loaded_config = config::load_config(temp_file.path());
    assert_eq!(loaded_config.shell_name, "fish");
    assert_eq!(loaded_config.settings.get("api_endpoint").unwrap(), "https://api.test.com");
    assert_eq!(loaded_config.settings.get("timeout").unwrap(), "60");
}

/// Test mock plugin functionality
#[test]
fn test_mock_plugin_initialization() {
    let config = PluginConfig::default();
    let result = MockPlugin::initialize(&config);
    assert!(result.is_ok());
}

#[test]
fn test_mock_plugin_pre_prompt_hook() {
    let config = PluginConfig::default();
    let _ = MockPlugin::initialize(&config);
    
    let mut context = ShellContext::new();
    let result = MockPlugin::pre_prompt_hook(&mut context);
    
    assert!(result.is_ok());
    assert!(context.contains_key("MOCK_PLUGIN_ACTIVE"));
    assert_eq!(context.get("MOCK_PLUGIN_ACTIVE").unwrap(), "true");
}

#[test]
fn test_mock_plugin_post_execute_hook() {
    let result = MockPlugin::post_execute_hook("test command", 0);
    assert!(result.is_ok());
    
    let result_fail = MockPlugin::post_execute_hook("failing command", 1);
    assert!(result_fail.is_err());
}

/// Test history management
#[test]
fn test_history_manager_creation() {
    let temp_dir = TempDir::new().unwrap();
    let history_path = temp_dir.path().join("history.json");
    
    let manager = HistoryManager::new(Some(history_path.to_str().unwrap()));
    assert!(manager.load().is_err()); // File doesn't exist yet
}

#[test]
fn test_history_add_and_retrieve() {
    let temp_dir = TempDir::new().unwrap();
    let history_path = temp_dir.path().join("history.json");
    
    let mut manager = HistoryManager::new(Some(history_path.to_str().unwrap()))
        .with_max_entries(100);
    
    // Add some history entries
    manager.add_entry("ls -la", 0, 150, None);
    manager.add_entry("git status", 0, 200, Some("AI suggested this".to_string()));
    manager.add_entry("cd /tmp", 0, 50, None);
    
    // Get recent entries
    let recent = manager.get_recent(2);
    assert_eq!(recent.len(), 2);
    assert_eq!(recent[0].command, "cd /tmp");
    assert_eq!(recent[1].command, "git status");
}

#[test]
fn test_history_search() {
    let temp_dir = TempDir::new().unwrap();
    let history_path = temp_dir.path().join("history.json");
    
    let mut manager = HistoryManager::new(Some(history_path.to_str().unwrap()))
        .with_max_entries(100);
    
    manager.add_entry("docker ps", 0, 300, None);
    manager.add_entry("docker images", 0, 250, None);
    manager.add_entry("git commit", 0, 180, None);
    
    let results = manager.search("docker");
    assert_eq!(results.len(), 2);
    
    let git_results = manager.search("git");
    assert_eq!(git_results.len(), 1);
}

#[test]
fn test_history_clear() {
    let temp_dir = TempDir::new().unwrap();
    let history_path = temp_dir.path().join("history.json");
    
    let mut manager = HistoryManager::new(Some(history_path.to_str().unwrap()))
        .with_max_entries(100);
    
    manager.add_entry("test command", 0, 100, None);
    manager.add_entry("another command", 0, 150, None);
    
    let clear_result = manager.clear();
    assert!(clear_result.is_ok());
    
    let recent = manager.get_recent(10);
    assert!(recent.is_empty());
}

/// Test context export
#[test]
fn test_context_creation_and_manipulation() {
    let mut context = ShellContext::new();
    
    context.insert("TEST_KEY".to_string(), "test_value".to_string());
    context.insert("ANOTHER_KEY".to_string(), "another_value".to_string());
    
    assert_eq!(context.get("TEST_KEY").unwrap(), "test_value");
    assert_eq!(context.get("ANOTHER_KEY").unwrap(), "another_value");
    assert_eq!(context.len(), 2);
}

/// Test AI integration placeholder
#[test]
fn test_ai_integration_creation() {
    let ai = shally::ai::AiIntegration::new();
    // Basic creation test - actual API calls would require credentials
    assert!(true);
}

/// Test CLI resolution
#[test]
fn test_config_resolution_with_explicit_path() {
    use std::fs;
    use tempfile::NamedTempFile;
    
    let mut temp_file = NamedTempFile::new().unwrap();
    let yaml_content = r#"shell_name: zsh"#;
    fs::write(&temp_file, yaml_content).unwrap();
    
    let path = config::resolve_config_path(Some(temp_file.path().to_str().unwrap()));
    assert!(path.is_some());
    assert_eq!(path.unwrap(), temp_file.path());
}

/// Test error handling
#[test]
fn test_error_types() {
    use shally::errors::ShallyError;
    
    let config_err = ShallyError::ConfigNotFound("/nonexistent/path.yaml".to_string());
    assert!(config_err.to_string().contains("/nonexistent/path.yaml"));
    
    let parse_err = ShallyError::ConfigParseError("Invalid YAML".to_string());
    assert!(parse_err.to_string().contains("Invalid YAML"));
}
