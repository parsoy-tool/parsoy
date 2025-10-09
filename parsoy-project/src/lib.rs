use parsoy_config::ParsoyConfig;

/// Create the directory for a new project.
pub fn create_project_root(
    target_dir: &std::path::Path,
    _project_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(target_dir)?;

    write_parsoy_config_file(target_dir)?;
    write_cargo_workspace_file(target_dir)?;
    
    Ok(())
}

fn write_parsoy_config_file(
    target_dir: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = target_dir.join(".parsoy.toml");

    let content = ParsoyConfig::default();

    let toml_string = toml::to_string_pretty(&content)?;

    std::fs::write(&config_path, toml_string).map_err(|e| e.into())
}

fn write_cargo_workspace_file(
    target_dir: &std::path::Path
) -> Result<(), Box<dyn std::error::Error>> {
    let cargo_path = target_dir.join("Cargo.toml");

    let content = "[workspace]\nresolver = \"3\"\nmembers = [\"\"]";
    
    std::fs::write(cargo_path, content)?;
    Ok(())
}