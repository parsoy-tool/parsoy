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
