use crate::{Template, Vcs};
use parsoy_template::get_template;
use std::path::PathBuf;

#[cfg(feature = "cli")]
pub async fn handle_new_command(
    project_name: String,
    template: Template,
    vcs: Vcs,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    todo!();
    Ok(())
}
