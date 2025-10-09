use serde::{Deserialize, Serialize};


/// Represents the table [project.grammar] in the configuration file.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GrammarConfig {
    /// Support Glob patterns
    pub source: Vec<String>,
}