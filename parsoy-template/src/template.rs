use crate::{Result, TemplateError};

/// Checks if a path element's name starts with an underscore (`_`) and
/// is a valid template name (i.e., not juste `_` or an empty string).
///
/// This function is used for filtering elements during recursive directory traversal.
///
/// # Arguments
///
/// * `template_path` - A reference to the path (file or directory) to validate.
/// 
/// # Returns
/// 
/// - `Ok(true)`: If the element is a valid template (e.g. `_component`).
/// - `Ok(false)`: If the element should be ignored (e.g., `src`, `/`, or `config.toml`).
/// 
/// # Examples
/// 
/// ```rust
/// use std::path::Path;
/// use parsoy_template::template::is_template_path;
/// 
/// assert_eq!(is_template_path(Path::new("./_item")).unwrap(), true);
/// assert_eq!(is_template_path(Path::new("./_")).unwrap(), false);
/// assert_eq!(is_template_path(Path::new("./src")).unwrap(), false);
/// assert_eq!(is_template_path(Path::new("./")).unwrap(), false);
/// ```
pub fn is_template_path<P: AsRef<std::path::Path>>(
    template_path: P,
) -> Result<bool> {
    let path_ref = template_path.as_ref();

    let name_os_str = match path_ref.file_name() {
        None => return Ok(false),
        Some(s) => s,
    };

    let name = match name_os_str.to_str() {
        Some(s) => s,
        None => return Err(TemplateError::NonUnicode),
    };

    Ok(name_starts_with_underscore(name))
}

/// Quickly verifies if a file or directory name begins with an underscore `_`.
///
/// # Arguments
///
/// * `name` - The name of the file or directory, guaranteed to be valid UTF-8.
///
/// # Returns
///
/// Returns `true` if the name starts with an underscore, otherwise `false`.
///
/// # Examples
///
/// ```rust
/// use parsoy_template::template::name_starts_with_underscore;
/// assert_eq!(name_starts_with_underscore("_config.toml"), true);
/// assert_eq!(name_starts_with_underscore("config.toml"), false); // Ignored
/// assert_eq!(name_starts_with_underscore("_"), false); // Ignored
/// assert_eq!(name_starts_with_underscore(""), false); // Ignored
/// ```
pub fn name_starts_with_underscore(name: &str) -> bool {
    if name.len() <= 1 {
        return false;
    }
    name.starts_with('_')
}
