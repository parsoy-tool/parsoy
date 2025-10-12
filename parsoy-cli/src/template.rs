use derive_more::Display;
use std::path::{Path, PathBuf};

#[macro_export]
macro_rules! write_template_file {
    () => {};
}

pub(crate) fn derive_filename<P: AsRef<Path>>(
    template_path: P,
) -> Result<String, TemplateError> {
    let path_ref = template_path.as_ref();

    let filename_os_str =
        path_ref.file_name().ok_or_else(|| {
            TemplateError::MissingFilename(
                path_ref.to_path_buf(),
            )
        })?;

    let filename =
        filename_os_str
            .to_str()
            .ok_or_else(|| {
                TemplateError::NonUnicode(
                    filename_os_str
                        .to_string_lossy()
                        .into_owned(),
                )
            })?;

    if filename.contains('_') {
        Ok(filename[1..].to_string())
    } else {
        Ok(filename.to_string())
    }
}

#[derive(Debug, Display)]
pub enum TemplateError {
    #[display(
        "The provided path does not contain a file name. [Path: {:#?}]",
        _0
    )]
    MissingFilename(PathBuf),
    #[display(
        "The file name is not a valid UTF-8 sequence. [Raw name: {:#?}]",
        _0
    )]
    NonUnicode(String),
}
