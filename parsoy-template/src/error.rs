use derive_more::{Display, Error, From};
use std::io;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, TemplateError>;

#[derive(Debug, Display, Error, From)]
pub enum TemplateError {
    #[display(
        "Template variable poorly formatted. [Error: {0:#?}], [file: {1:#?}]",
        _0,
        _1
    )]
    Parsing(String, PathBuf),
    #[display(
        "Missing variable. The key [{0:#?}] was not found in the context for the file: [{1:#?}]",
        _0,
        _1
    )]
    MissingVariable(String, PathBuf),
    // Version fonctionnelle : utilise {0} et {1} pour les arguments positionnels
    // et _0, _1 pour référencer les champs.
    #[display(
        "Read/write error on the path. [Path: {0:#?}], [Error: {1:#?}]",
        _0,
        _1
    )]
    #[from]
    Io(PathBuf, io::Error),

    #[from]
    IoGeneric(io::Error),
}

pub fn handle_error<E>(error: E)
where
    E: std::fmt::Display,
{
    eprintln!("{error}");
    ::std::process::exit(1)
}
