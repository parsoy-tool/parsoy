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

    Ok(())
}
