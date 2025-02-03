use crate::logging::*;
use serde_json;
use std::{
    fs::*,
    io::Write,
};

impl Logger {
    /// # Creates a `Logger` instance from a template file.
    pub fn from_template(path: &str) -> Logger {
        let file = read_to_string(path)
            .expect("Unable to read file!");
        let logger: Logger = serde_json::from_str(&file)
            .expect("Invalid JSON file!");

        return logger;
    }

    /// # Saves a `Logger` template file.
    pub fn save_template(&self, path: &str) {
        let json = serde_json::to_string_pretty(self)
            .expect("Failed to serialize");

        let mut file = File::create(path)
            .expect("Unable to create file");

        file.write_all(json.as_bytes())
            .expect("Unable to write data");
    }
}
