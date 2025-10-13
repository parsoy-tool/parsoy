use walkdir::WalkDir;

use crate::{Error, Result};
use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};

pub type TemplateCtxt = HashMap<String, String>;

pub struct Template {
    pub root_path: PathBuf,
    pub items: Vec<TemplateItem>,
}

impl Template {
    fn check_root_existence(
        root_path: &Path,
    ) -> Result<()> {
        match std::fs::metadata(root_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn find_items_recursively(
        root_path: &Path,
    ) -> Result<Vec<TemplateItem>> {
        let mut items = Vec::new();
        for entry_result in WalkDir::new(root_path) {
            let entry =
                entry_result.map_err(Error::WalkDir)?;
            let path = entry.path();

            if path == root_path {
                continue;
            }

            if let Some(item) =
                TemplateItem::try_from_path(path)?
            {
                items.push(item);
            }
        }
        Ok(items)
    }
}

#[derive(Debug, Clone)]
pub struct TemplateItem {
    pub source_path: PathBuf,
    pub final_name: String,
    pub is_dir: bool,
}

impl TemplateItem {
    fn name_starts_with_underscore(name: &str) -> bool {
        if name.len() <= 1 {
            return false;
        }
        name.starts_with('_')
    }

    fn get_name_os_str<'a>(
        path: &'a Path,
    ) -> Option<&'a OsStr> {
        path.file_name()
    }

    fn validate_and_extract_name<'a>(
        name_os_str: &'a OsStr,
    ) -> Result<Option<&'a str>> {
        let name = match name_os_str.to_str() {
            Some(s) => s,
            None => return Err(Error::NonUnicode),
        };

        if Self::name_starts_with_underscore(name) {
            Ok(Some(name))
        } else {
            Ok(None) // Ignored
        }
    }

    fn derive_final_name(name: &str) -> String {
        name.strip_prefix('_')
            .unwrap_or(name)
            .to_string()
    }

    pub fn try_from_path(
        path: &Path,
    ) -> Result<Option<Self>> {
        let name_os_str = match Self::get_name_os_str(path)
        {
            Some(s) => s,
            None => return Ok(None), // Root path ignored,
        };
        let name = match Self::validate_and_extract_name(
            name_os_str,
        )? {
            Some(n) => n,
            None => return Ok(None), // filted name without underscore.
        };

        let final_name = Self::derive_final_name(name);

        Ok(Some(TemplateItem {
            source_path: path.to_path_buf(),
            final_name,
            is_dir: path.is_dir(),
        }))
    }
}
