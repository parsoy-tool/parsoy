use crate::{Error, Item, Result};
use std::path::Path;
use walkdir::{IntoIter, WalkDir};

pub struct Template {
    inner: IntoIter,
}

impl Template {}

impl Iterator for Template {
    type Item = Result<Item>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = match self.inner.next() {
                Some(Ok(e)) => e,
                Some(Err(e)) => {
                    return Some(Err(Error::Io(
                        e.into_io_error()?,
                    )));
                }
                None => return None,
            };

            let path = entry.into_path();

            let file_name = match Item::extract_name_str(&path) {
                Err(e) => return Some(Err(e)),
                Ok(name) => name,
            };

            match Item::from_path(path) {
                Ok(item) => return Some(Ok(item)),
                Err(e) => return Some(Err(e)),
            }
        }
    }
}
