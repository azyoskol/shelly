//! AI Core module for Shally
//! Provides AI decision making, prompt building, and response parsing

mod decision;
mod prompt;
mod engine;

pub use decision::AiDecision;
pub use prompt::PromptBuilder;
pub use engine::AiEngine;
