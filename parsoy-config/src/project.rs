//!

mod grammar;
pub use self::grammar::GrammarConfig;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectConfig {
    pub grammar: GrammarConfig,
}
