use std::path::PathBuf;

use crate::{ProjectTemplate, handle_error};

pub async fn run_new_command(
    project_name: String,
    template: ProjectTemplate,
) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = PathBuf::from(&project_name);

    std::fs::create_dir_all(&project_dir).unwrap();

    match template {
        ProjectTemplate::Compiler => run_new_compiler().await,
        ProjectTemplate::Interpreter => run_new_interpreter().await,
    }
}

async fn run_new_interpreter() -> Result<(), Box<dyn std::error::Error>> {
todo!();
}

async fn run_new_compiler() -> Result<(), Box<dyn std::error::Error>> {
todo!();
}
