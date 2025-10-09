use std::path::{Path, PathBuf};

pub fn run_new_command(project_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = PathBuf::from(&project_name);


    parsoy_project::create_project_root(&target_dir, &project_name)?;
    Ok(())
}