use std::{fs::{File, OpenOptions,}, io::{Write, Result}};

pub(crate) fn is_file_writable(path: &str) -> bool {
    match File::create(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub(crate) fn overwrite_file(path: &str, content: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?;

    file.write_all(content.as_bytes())?;
    Ok(())
}

pub(crate) fn append_to_file(path: &str, content: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)?;

    file.write_all(content.as_bytes())?;
    Ok(())
}
