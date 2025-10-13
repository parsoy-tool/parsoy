use derive_more::From;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    MissingFilename,
    NonUnicode,

    #[from]
    WalkDir(walkdir::Error),

    #[from]
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
