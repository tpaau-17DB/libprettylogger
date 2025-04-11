use std::{
    env::{var, vars},
    fs::{File, OpenOptions},
    io::Write,
    path::Path
};

use crate::Error;

/// Returns `true` if a file exists and is writable and `false` otherwise.
pub(crate) fn is_file_writable(path: &str) -> bool {
    let file = Path::new(path);
    if OpenOptions::new().write(true).open(file).is_ok() {
        return true;
    }
    else {
        return File::create(path).is_ok();
    }
}

/// Writes contents to a file overwriting it.
pub(crate) fn overwrite_file(path: &str, content: &str) -> Result<(), Error> {
    match OpenOptions::new()
         .write(true)
        .truncate(true)
        .open(path) {
        Ok(mut open_file) => {
            match open_file.write_all(content.as_bytes()) {
                Ok(_) => { Ok(()) },
                Err(e) => { Err(Error::new(&e.to_string())) }
            }
        },
        Err(e) => { Err(Error::new(&e.to_string())) }
    }

}

pub(crate) fn append_to_file(path: &str, content: &str) -> Result<(), Error> {
    match OpenOptions::new()
        .append(true)
        .open(path) {
            Ok(mut file) => {
                match file.write_all(content.as_bytes()) {
                    Ok(_) => { Ok(()) },
                    Err(e) => { Err(Error::new(&e.to_string())) }
                }
            },
            Err(e) => {
                Err(Error::new(&e.to_string()))
            }
        }
}

pub(crate) fn expand_tilde(path: &str) -> String {
    if path.starts_with("~") {
        let home_dir = var("HOME")
            .or_else(|_| var("USERPROFILE"))
            .unwrap_or_else(|_| "/".to_string());
        path.replace("~", &home_dir)
    } else {
        String::from(path)
    }
}

pub(crate) fn expand_env_vars(path: &str) -> String {
    let mut expanded_path = path.to_string();

    for (key, value) in vars() {
        let var_name = format!("${}", key);
        if expanded_path.contains(&var_name) {
            expanded_path = expanded_path.replace(&var_name, &value);
        }
    }
    expanded_path
}
