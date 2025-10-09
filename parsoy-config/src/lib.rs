mod project;
use crate::project::ProjectConfig;

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParsoyConfig {
    pub project: ProjectConfig,
}

pub fn load_config_from_file<P: AsRef<Path>>(
    config_path: P,
) -> Result<ParsoyConfig, Box<dyn std::error::Error>> {
    let path = config_path.as_ref();

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Ok(ParsoyConfig::default());
        },
        Err(e) => {
            return Err(e.into());
        }
    };

    let config: ParsoyConfig = toml::from_str(&content)?;

    Ok(config)
}
