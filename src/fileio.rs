use std::{env::{var, vars}, fs::{File, OpenOptions,}, io::{Write, Result}};

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

pub fn expand_tilde(path: &str) -> String {
    if path.starts_with("~") {
        let home_dir = var("HOME")
            .or_else(|_| var("USERPROFILE"))
            .unwrap_or_else(|_| "/".to_string());
        path.replace("~", &home_dir)
    } else {
        path.to_string()
    }
}

pub fn expand_env_vars(path: &str) -> String {
    let mut expanded_path = path.to_string();

    for (key, value) in vars() {
        let var_name = format!("${}", key);
        if expanded_path.contains(&var_name) {
            expanded_path = expanded_path.replace(&var_name, &value);
        }
    }

    return expanded_path
}
