use crate::{Template, Vcs, write_template_file};
use std::path::PathBuf;

#[cfg(feature = "cli")]
pub async fn handle_new_command(
    project_name: String,
    _template: Template,
    _vcs: Vcs,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_path =
        PathBuf::from("./").join(&project_name);

    generate_root_files(
        &target_path,
        project_name,
        verbose,
    )
    .await?;

    Ok(())
}

<<<<<<< HEAD
pub(self) async fn setup_project_root()
-> Result<(), Box<dyn std::error::Error>> {
    todo!()
}

pub(self) async fn ensure_directory_exists()
-> Result<(), Box<dyn std::error::Error>> {
    todo!()
}

#[allow(unused_must_use)]
pub(self) async fn generate_root_files(
    target_path: &PathBuf,
    project_name: impl Into<String>,
    _verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
   
    Ok(())
=======
async fn run_new_interpreter() -> Result<(), Box<dyn std::error::Error>> {
todo!();
}

async fn run_new_compiler() -> Result<(), Box<dyn std::error::Error>> {
todo!();
>>>>>>> b7813ddb84355b7548cc053776c08c4fb21e4b61
}
