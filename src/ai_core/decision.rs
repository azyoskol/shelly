/// Represents a decision made by the AI engine on whether to intercept
#[derive(Debug, Clone, PartialEq)]
pub struct AiDecision {
    /// Whether the AI should intercept and offer help
    pub should_intercept: bool,
    /// Reason for interception (e.g., "command_failed", "command_not_found")
    pub reason: &'static str,
    /// Priority of the suggestion (higher = more urgent)
    pub suggestion_priority: u8,
}

impl AiDecision {
    /// Create a decision to NOT intercept
    pub fn no_intercept() -> Self {
        AiDecision {
            should_intercept: false,
            reason: "",
            suggestion_priority: 0,
        }
    }

    /// Create a decision to intercept with given reason and priority
    pub fn intercept(reason: &'static str, priority: u8) -> Self {
        AiDecision {
            should_intercept: true,
            reason,
            suggestion_priority: priority,
        }
    }
}
