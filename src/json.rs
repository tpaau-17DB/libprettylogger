use crate::*;
use serde_json;
use std::{
    fs::*,
    io::Write,
};

impl Logger {
    /// Creates a `Logger` instance from a template file.
    ///
    /// [Logger templates documentation](https://github.com/tpaau-17DB/libprettylogger?tab=readme-ov-file#logger-templates)
    /// # Deserializing `Logger` from a `json` file:
    ///
    /// ```ignore
    /// # use prettylogger::logger::Logger;
    /// let mut logger = Logger::from_template("/path/to/template.json");
    /// ```
    pub fn from_template(path: &str) -> Logger {
        let file = read_to_string(path)
            .expect("Unable to read file!");
        let mut logger: Logger = serde_json::from_str(&file)
            .expect("Invalid JSON file!");
        logger.log_count += 1;


        return logger;
    }

    /// Saves a `Logger` to template file.
    ///
    /// [Logger templates documentation](https://github.com/tpaau-17DB/libprettylogger?tab=readme-ov-file#logger-templates)
    ///
    /// # Serializing `Logger` into a `json` file
    /// ```ignore
    /// # use prettylogger::logger::Logger;
    /// let mut logger = Logger::default(); // Create a default `Logger`
    /// logger.save_template("/path/to/template.json");
    /// ```
    pub fn save_template(&self, path: &str) {
        let json = serde_json::to_string_pretty(self)
            .expect("Failed to serialize");

        let mut file = File::create(path)
            .expect("Unable to create file");

        file.write_all(json.as_bytes())
            .expect("Unable to write data");
    }
}
