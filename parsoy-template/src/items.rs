use crate::error::{Error, Result};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Item {
    pub source_path: PathBuf,
    pub name_pattern: String,
    pub final_name: String,
    pub is_dir: bool,
}

impl Item {
    pub fn from_path(source_path: PathBuf) -> Result<Self> {
        match Self::try_from_path(source_path) {
            Ok(i) => Ok(i),
            Err(e) => return Err(e),
        }
    }

    fn try_from_path(source_path: PathBuf) -> Result<Self> {
        let name_str =
            Self::extract_name_str(&source_path)?;

        Self::validate_name(name_str)?;

        let name_pattern_owned = name_str.to_string();

        let final_name = name_str
        .strip_prefix("_")
                    .expect("Validation ensures the name starts with an underscore.")
                    .to_string();

        let is_dir = source_path.is_dir();

        Ok(Item {
            source_path,
            name_pattern: name_pattern_owned,
            final_name,
            is_dir,
        })
    }

    pub fn validate_name<'a>(
        name: &'a str,
    ) -> Result<&'a str> {
        if Self::is_fully_valid_name_format(name) {
            Ok(name)
        } else {
            Err(Error::Name(
                "Name validation failed. Must start with a single underscore and be followed by a letter (e.g., `_file`). Cannot be `_` or start with `__`.",
            ))
        }
    }

    pub fn extract_name_str<'a>(
        path: &'a Path,
    ) -> Result<&'a str> {
        if let Some(name_os) = path.file_name() {
            if let Some(name) = name_os.to_str() {
                Ok(name)
            } else {
                Err(Error::NonUnicode)
            }
        } else {
            Err(Error::Path("Path has no file name"))
        }
    }

    pub fn is_fully_valid_name_format(name: &str) -> bool {
        // 1. Must start with '_'
        name.starts_with('_')
        // 2. Must be longer than `_` (length > 1)
        && name.len() > 1
        // 3. Must not start with `__`
        && !name.starts_with("__")
        // 4. Must have a letter after the underscore
        && name.chars()
            .nth(1) // Get the second character (index 1)
            .map_or(false, |c| c.is_alphabetic())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_names_pass() {
        assert!(
            Item::is_fully_valid_name_format("_file"),
            "Should pass: _file"
        );
        assert!(
            Item::is_fully_valid_name_format("_Template"),
            "Should pass: _Template"
        );
        assert!(
            Item::is_fully_valid_name_format("_a"),
            "Should pass: _a"
        );
        assert!(
            Item::is_fully_valid_name_format("_z_test"),
            "Should pass: _z_test"
        );
        assert!(
            Item::is_fully_valid_name_format("_Élément"),
            "Should pass: _Élément"
        );
    }

    #[test]
    fn invalid_names_fail() {
        assert!(
            !Item::is_fully_valid_name_format("_"),
            "Should fail: just _"
        );
        assert!(
            !Item::is_fully_valid_name_format("__file"),
            "Should fail: starts with __"
        );
        assert!(
            !Item::is_fully_valid_name_format("___file"),
            "Should fail: starts with ___"
        );
        assert!(
            !Item::is_fully_valid_name_format("file"),
            "Should fail: no underscore"
        );
        assert!(
            !Item::is_fully_valid_name_format("_1file"),
            "Should fail: followed by digit"
        );
        assert!(
            !Item::is_fully_valid_name_format("_/file"),
            "Should fail: followed by slash"
        );
        assert!(
            !Item::is_fully_valid_name_format("_."),
            "Should fail: followed by dot"
        );
        assert!(
            !Item::is_fully_valid_name_format(
                "_{variable}"
            ),
            "Should fail: followed by curly brace"
        );
    }

    #[test]
    fn validate_name_returns_ok_on_valid() {
        let name = "_valid_name";
        assert!(Item::validate_name(name).is_ok());
        assert_eq!(
            Item::validate_name(name).unwrap(),
            name
        );
    }
}
