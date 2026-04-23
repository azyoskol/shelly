// tests/ai_and_history_tests.rs
// TDD Tests for AI Agent logic and Smart History features

use shally::core::types::{ShellContext, CommandResult};
use shally::ai_core::{AiDecision, AiEngine, PromptBuilder};
use shally::smart_history::{HistoryEntry, SmartHistory, SearchRank};
use shally::errors::ShallyError;

#[cfg(test)]
mod ai_agent_tests {
    use super::*;

    // --- Тесты для AI Decision Logic (Нужно ли вмешиваться?) ---

    #[test]
    fn test_ai_decision_on_error() {
        // Идея: Если команда упала с ошибкой, AI должен предложить решение
        let ctx = ShellContext {
            last_command: Some("git commmit -m 'fix'".to_string()),
            last_output: Some("error: pathspec 'commmit' did not match any file(s) known to git".to_string()),
            exit_code: Some(1),
            cwd: "/home/user/project".to_string(),
            shell: "zsh".to_string(),
        };

        let decision = AiEngine::should_intercept(&ctx);
        
        assert!(decision.should_intercept);
        assert_eq!(decision.reason, "command_failed");
        assert!(decision.suggestion_priority > 0);
    }

    #[test]
    fn test_ai_decision_on_unknown_command() {
        // Идея: Если команда не найдена, AI должен предложить установку или альтернативу
        let ctx = ShellContext {
            last_command: Some("docker-composee up".to_string()),
            last_output: Some("command not found: docker-composee".to_string()),
            exit_code: Some(127),
            cwd: "/home/user".to_string(),
            shell: "fish".to_string(),
        };

        let decision = AiEngine::should_intercept(&ctx);
        
        assert!(decision.should_intercept);
        assert_eq!(decision.reason, "command_not_found");
    }

    #[test]
    fn test_ai_decision_no_intercept_on_success() {
        // Идея: Не мешать пользователю, если все работает хорошо
        let ctx = ShellContext {
            last_command: Some("ls -la".to_string()),
            last_output: Some("total 48...".to_string()),
            exit_code: Some(0),
            cwd: "/home/user".to_string(),
            shell: "bash".to_string(),
        };

        let decision = AiEngine::should_intercept(&ctx);
        
        assert!(!decision.should_intercept);
    }

    // --- Тесты для Prompt Building (Контекстуализация) ---

    #[test]
    fn test_prompt_builder_includes_context() {
        let ctx = ShellContext {
            last_command: Some("rm -rf /tmp/test".to_string()),
            last_output: None,
            exit_code: None,
            cwd: "/home/user/dangerous_zone".to_string(),
            shell: "zsh".to_string(),
        };

        let prompt = PromptBuilder::build_help_prompt(&ctx);

        assert!(prompt.contains("rm -rf"));
        assert!(prompt.contains("/home/user/dangerous_zone"));
        assert!(prompt.contains("zsh"));
        assert!(prompt.contains("fix") || prompt.contains("explain"));
    }

    #[test]
    fn test_prompt_builder_handles_missing_data() {
        let ctx = ShellContext {
            last_command: None,
            last_output: None,
            exit_code: None,
            cwd: "/".to_string(),
            shell: "unknown".to_string(),
        };

        // Не должно паниковать, даже если контекст пустой
        let prompt = PromptBuilder::build_help_prompt(&ctx);
        assert!(!prompt.is_empty());
    }

    // --- Тесты для Response Parsing (Понимание ответа AI) ---

    #[test]
    fn test_parse_ai_suggestion_command() {
        // AI ответил командой
        let response = "```bash\ngit commit -m 'fix'\n```";
        let parsed = AiEngine::parse_suggestion(response);
        
        assert!(parsed.is_some());
        let cmd = parsed.unwrap();
        assert_eq!(cmd.action, "execute");
        assert!(cmd.content.contains("git commit"));
    }

    #[test]
    fn test_parse_ai_explanation_text() {
        // AI ответил объяснением
        let response = "Вы забыли букву 't' в слове commit. Правильная команда: ...";
        let parsed = AiEngine::parse_suggestion(response);
        
        assert!(parsed.is_some());
        let cmd = parsed.unwrap();
        assert_eq!(cmd.action, "explain");
    }
}

#[cfg(test)]
mod smart_history_tests {
    use super::*;
    use std::collections::HashMap;

    // --- Тесты для Умного Поиска (не просто grep) ---

    #[test]
    fn test_history_search_ranking_by_frequency() {
        let mut history = SmartHistory::new();
        
        // Добавляем команды с разной частотой
        history.add(HistoryEntry::new("git status", "/proj", 0));
        history.add(HistoryEntry::new("git status", "/proj", 0));
        history.add(HistoryEntry::new("git status", "/proj", 0));
        history.add(HistoryEntry::new("git pull", "/proj", 0));
        
        let results = history.search("git st", Some("/proj"));
        
        assert!(!results.is_empty());
        // git status должен быть выше из-за частоты
        assert_eq!(results[0].command, "git status");
        assert!(results[0].score > results.iter().find(|r| r.command == "git pull").unwrap().score);
    }

    #[test]
    fn test_history_search_context_awareness() {
        let mut history = SmartHistory::new();
        
        // Одинаковые команды в разных директориях
        history.add(HistoryEntry::new("npm install", "/frontend", 0));
        history.add(HistoryEntry::new("pip install", "/backend", 0));
        
        // Поиск из директории frontend
        let results = history.search("install", Some("/frontend"));
        
        assert_eq!(results[0].command, "npm install");
    }

    #[test]
    fn test_history_fuzzy_matching() {
        let mut history = SmartHistory::new();
        history.add(HistoryEntry::new("kubectl get pods", "/dev", 0));
        
        // Опечатка в запросе
        let results = history.search("kubctl get pds", None);
        
        assert!(!results.is_empty());
        assert_eq!(results[0].command, "kubectl get pods");
    }

    // --- Тесты для Статистики и Аналитики ---

    #[test]
    fn test_get_frequent_commands() {
        let mut history = SmartHistory::new();
        history.add(HistoryEntry::new("ls", "/", 0));
        history.add(HistoryEntry::new("cd ..", "/", 0));
        history.add(HistoryEntry::new("ls", "/", 0));
        
        let stats = history.get_frequent_commands(1);
        
        assert_eq!(stats.len(), 1);
        assert_eq!(stats[0].0, "ls");
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_full_fix_flow_simulation() {
        // Симуляция полного цикла: Ошибка -> Решение AI -> Исправление
        
        // 1. Контекст ошибки
        let ctx = ShellContext {
            last_command: Some("doker run hello-world".to_string()),
            last_output: Some("command not found: doker".to_string()),
            exit_code: Some(127),
            cwd: "/home".to_string(),
            shell: "zsh".to_string(),
        };

        // 2. Проверка необходимости вмешательства
        let decision = AiEngine::should_intercept(&ctx);
        assert!(decision.should_intercept);

        // 3. Генерация промпта
        let prompt = PromptBuilder::build_help_prompt(&ctx);
        assert!(prompt.contains("doker"));

        // 4. Симуляция ответа от "мока" AI (в реальности тут был бы вызов API)
        let mock_ai_response = "Кажется, вы опечатались. Правильная команда: `docker run hello-world`";
        
        // 5. Парсинг ответа
        let suggestion = AiEngine::parse_suggestion(mock_ai_response);
        assert!(suggestion.is_some());
        assert_eq!(suggestion.unwrap().action, "explain");
    }

    #[test]
    fn test_history_learning_from_correction() {
        let mut history = SmartHistory::new();
        
        // Пользователь ввел команду с ошибкой, потом исправил
        history.add(HistoryEntry::new("gitt status", "/proj", 0)); // Ошибка
        // Предположим, AI подсказал, и пользователь ввел верно:
        history.add(HistoryEntry::new("git status", "/proj", 0)); 
        history.add(HistoryEntry::new("git status", "/proj", 0)); 

        // При поиске "gitt" (опечатка) мы должны найти "git status" благодаря нечеткому поиску
        let results = history.search("gitt stat", Some("/proj"));
        
        assert!(!results.is_empty());
        assert_eq!(results[0].command, "git status");
    }
}
