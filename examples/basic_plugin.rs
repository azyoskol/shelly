// examples/basic_plugin.rs
// Example: Basic plugin implementation for Shally Framework
//
// This example demonstrates how to create a simple plugin that implements
// the ShellPlugin trait and integrates with the Shally framework.

use shally::core::{PluginConfig, ShellContext, ShellPlugin};

/// A simple plugin that displays the current directory
pub struct CurrentDirPlugin;

impl ShellPlugin for CurrentDirPlugin {
    fn name() -> &'static str {
        "current_dir"
    }

    fn initialize(config: &PluginConfig) -> Result<(), String> {
        println!("[{}] Initializing with shell: {}", Self::name(), config.shell_name);
        Ok(())
    }

    fn pre_prompt_hook(context: &mut ShellContext) -> Result<(), String> {
        // Add current directory to context
        if let Ok(cwd) = std::env::current_dir() {
            context.insert(
                "CURRENT_DIR".to_string(),
                cwd.to_string_lossy().to_string(),
            );
            println!("[{}] Current directory: {}", Self::name(), cwd.display());
        }
        Ok(())
    }

    fn post_execute_hook(_command: &str, exit_code: i32) -> Result<(), String> {
        if exit_code == 0 {
            println!("[{}] Command executed successfully", Self::name());
        } else {
            println!("[{}] Command failed with exit code: {}", Self::name(), exit_code);
        }
        Ok(())
    }
}

/// A plugin that tracks command execution time
pub struct TimingPlugin;

impl ShellPlugin for TimingPlugin {
    fn name() -> &'static str {
        "timing"
    }

    fn initialize(config: &PluginConfig) -> Result<(), String> {
        println!("[{}] Initializing timing plugin", Self::name());
        Ok(())
    }

    fn pre_prompt_hook(context: &mut ShellContext) -> Result<(), String> {
        // Record prompt display time
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        context.insert("PROMPT_TIME".to_string(), timestamp.to_string());
        Ok(())
    }

    fn post_execute_hook(command: &str, exit_code: i32) -> Result<(), String> {
        println!("[{}] Executed '{}' with exit code {}", Self::name(), command, exit_code);
        Ok(())
    }
}

fn main() {
    println!("=== Shally Framework Plugin Example ===\n");

    // Create a sample configuration
    let config = PluginConfig {
        shell_name: "zsh".to_string(),
        settings: [(
            "enable_timing".to_string(),
            "true".to_string(),
        )].iter().cloned().collect(),
    };

    // Initialize plugins
    println!("Initializing plugins...\n");
    
    if let Err(e) = CurrentDirPlugin::initialize(&config) {
        eprintln!("Failed to initialize CurrentDirPlugin: {}", e);
        return;
    }

    if let Err(e) = TimingPlugin::initialize(&config) {
        eprintln!("Failed to initialize TimingPlugin: {}", e);
        return;
    }

    println!("\n--- Running Pre-Prompt Hook ---\n");
    
    let mut context = ShellContext::new();
    
    // Execute pre-prompt hooks
    if let Err(e) = CurrentDirPlugin::pre_prompt_hook(&mut context) {
        eprintln!("CurrentDirPlugin pre-prompt hook failed: {}", e);
    }

    if let Err(e) = TimingPlugin::pre_prompt_hook(&mut context) {
        eprintln!("TimingPlugin pre-prompt hook failed: {}", e);
    }

    println!("\nContext after pre-prompt hooks:");
    for (key, value) in &context {
        println!("  {}: {}", key, value);
    }

    println!("\n--- Simulating Command Execution ---\n");
    
    // Simulate command execution
    let _ = CurrentDirPlugin::post_execute_hook("ls -la", 0);
    let _ = TimingPlugin::post_execute_hook("git status", 0);

    println!("\n=== Example Complete ===");
}
