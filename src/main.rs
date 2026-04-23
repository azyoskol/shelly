use shellai::plugin::{PluginConfig, ShellContext, ShellPlugin, mock_plugin::MockPlugin};
use shellai::config;
use shellai::zsh;
use shellai::fish;
use shellai::starship;
use shellai::ai::AiIntegration;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Resolve config from --config flag, env var, or default locations
    let explicit_config = args.iter()
        .position(|a| a == "--config")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str());

    let base_config = if let Some(path) = config::resolve_config_path(explicit_config) {
        match config::load_config_from_path(&std::path::PathBuf::from(path.clone())) {
            Some(c) => {
                println!("Configuration loaded from: {}", path.display());
                c
            }
            None => {
                eprintln!("Warning: Could not parse config file {}, using defaults.", path.display());
                PluginConfig::default()
            }
        }
    } else {
        PluginConfig::default()
    };

    if args.len() > 2 && args[1] == "--hook" {
        let hook_type = args[2].as_str();
        let mut context = ShellContext::new();
        let config = PluginConfig {
            shell_name: "zsh".to_string(),
            settings: base_config.settings.clone(),
            ..base_config.clone()
        };
        let _ = MockPlugin::initialize(&config);

        match hook_type {
            "precmd" => {
                let _ = zsh::precmd::<MockPlugin>(&mut context);
            }
            "preexec" => {
                let command = args.get(3).map(|s| s.as_str()).unwrap_or("");
                let _ = zsh::preexec::<MockPlugin>(command, 0, &mut context);
            }
            _ => {
                eprintln!("Unknown hook type: {}", hook_type);
            }
        }
        return;
    }

    if args.len() > 2 && args[1] == "--fish-hook" {
        let hook_type = args[2].as_str();
        let mut context = ShellContext::new();
        let config = PluginConfig {
            shell_name: "fish".to_string(),
            settings: base_config.settings.clone(),
            ..base_config.clone()
        };
        let _ = MockPlugin::initialize(&config);

        match hook_type {
            "prompt" => {
                let _ = fish::prompt::<MockPlugin>(&mut context);
            }
            "command_not_found" => {
                let command = args.get(3).map(|s| s.as_str()).unwrap_or("");
                let _ = fish::command_not_found::<MockPlugin>(command, &mut context);
            }
            _ => {
                eprintln!("Unknown fish hook type: {}", hook_type);
            }
        }
        return;
    }

    if args.len() > 1 && args[1] == "--install" {
        let binary_path = args.get(2).map(|s| s.as_str()).unwrap_or("shellai");
        println!("{}", zsh::generate_zshrc_snippet(binary_path));
        return;
    }

    if args.len() > 1 && args[1] == "--fish-install" {
        let binary_path = args.get(2).map(|s| s.as_str()).unwrap_or("shellai");
        println!("{}", fish::generate_config_snippet(binary_path));
        return;
    }

    if args.len() > 1 && args[1] == "--export-context" {
        let mut context = ShellContext::new();
        context.insert("CURRENT_DIR".to_string(), env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default());
        let exports = starship::export_context(&context);
        for (key, value) in exports {
            println!("{}={}", key, value);
        }
        return;
    }

    if args.len() > 1 && args[1] == "--ai-config" {
        let endpoint = args.get(2).map(|s| s.as_str()).unwrap_or("https://api.openai.com/v1/chat/completions");
        let api_key = args.get(3).map(|s| s.as_str()).unwrap_or("");
        let model = args.get(4).map(|s| s.as_str()).unwrap_or("gpt-3.5-turbo");

        let mut ai = AiIntegration::new();
        ai.with_config(endpoint, api_key).with_model(model);

        let context = ShellContext::new();
        let history: Vec<String> = vec![];

        match ai.suggest_command(&context, &history) {
            Ok(suggestion) => {
                println!("Command: {}", suggestion.command);
                if let Some(exp) = suggestion.explanation {
                    println!("Explanation: {}", exp);
                }
                println!("Confidence: {:.2}", suggestion.confidence);
            }
            Err(e) => eprintln!("AI suggestion failed: {}", e),
        }
        return;
    }

    println!("ShellAI Framework: Initializing core...");

    let config = base_config.clone();
    println!("Configuration loaded for shell: {}", config.shell_name);

    let mut context = ShellContext::new();

    if let Ok(_) = MockPlugin::initialize(&config) {
        println!("✅ MockPlugin Initialized successfully.");
    } else {
        eprintln!("❌ Initialization failed.");
        return;
    }

    match MockPlugin::pre_prompt_hook(&mut context) {
        Ok(_) => println!("\n✨ Pre-Prompt Hook executed. Context updated: {:?}", context),
        Err(e) => eprintln!("Error during pre-hook: {}", e),
    }

    println!("\nShellAI Framework Initialized successfully.");
}
