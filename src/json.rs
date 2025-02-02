use crate::logging::*;
use std::fs;
use serde_json;
use std::fs::File;
use std::io::Write;

impl Logger {
    pub fn from_template(path: &str) -> Logger {
        let file = fs::read_to_string(path)
            .expect("Unable to read file!");
        let logger: Logger = serde_json::from_str(&file)
            .expect("Invalid JSON file!");

        return logger;
    }

    pub fn save_template(&self, path: &str) {
        let json = serde_json::to_string_pretty(self)
            .expect("Failed to serialize");

        let mut file = File::create(path)
            .expect("Unable to create file");

        file.write_all(json.as_bytes())
            .expect("Unable to write data");
    }
}
