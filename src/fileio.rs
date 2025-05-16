use std::{
    fs::OpenOptions,
    io::Write,
};

use crate::Error;

/// Writes contents to a file overwriting it.
pub(crate) fn overwrite_file(path: &str, content: &str) -> Result<(), Error> {
    match OpenOptions::new()
         .write(true)
        .truncate(true)
        .open(path) {
        Ok(mut open_file) => {
            match open_file.write_all(content.as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::new(&e.to_string()))
            }
        },
        Err(e) => Err(Error::new(&e.to_string()))
    }

}

/// Appends a string to a file at given path.
pub(crate) fn append_to_file(path: &str, content: &str) -> Result<(), Error> {
    match OpenOptions::new()
        .append(true)
        .open(path) {
            Ok(mut file) => {
                match file.write_all(content.as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Error::new(&e.to_string()))
                }
            },
            Err(e) => Err(Error::new(&e.to_string()))
        }
}
