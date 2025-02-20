use crate::*;
use serde_json;
use std::{
    fs::*,
    io::Write,
};
use crate::fileio::{expand_env_vars, expand_tilde};

impl Logger {
    /// Creates a `Logger` instance from a template file.
    ///
    /// Automatically expands env variables.
    ///
    /// [Logger templates documentation.](https://github.com/tpaau-17DB/libprettylogger?tab=readme-ov-file#logger-templates)
    ///
    /// # Example
    /// ```
    /// # use prettylogger::Logger;
    /// # let mut path = std::env::temp_dir();
    /// # path.push("libprettylogger-tests/from-template.json");
    /// # let path = &path.to_str().unwrap().to_string();
    /// # Logger::default().save_template(path);
    /// let mut logger = Logger::from_template(path);
    /// ```
    pub fn from_template(path: &str) -> Logger {
        let path = expand_env_vars(&expand_tilde(path));

        let file = read_to_string(path)
            .expect("Unable to read file!");
        let mut logger: Logger = serde_json::from_str(&file)
            .expect("Invalid JSON file!");

        logger.log_count += 1;
        logger.show_datetime = logger.log_format.contains("%d");

        return logger;
    }

    /// Saves a `Logger` to template file.
    ///
    /// Automatically expands env variables.
    ///
    /// [Logger templates documentation](https://github.com/tpaau-17DB/libprettylogger?tab=readme-ov-file#logger-templates)
    ///
    /// # Example
    /// ```
    /// # use prettylogger::Logger;
    /// # let mut path = std::env::temp_dir();
    /// # path.push("libprettylogger-tests/from-template.json");
    /// # let path = &path.to_str().unwrap().to_string();
    /// let mut logger = Logger::default();
    /// logger.save_template(path);
    /// ```
    pub fn save_template(&self, path: &str) {
        let path = expand_env_vars(&expand_tilde(path));

        let json = serde_json::to_string_pretty(self)
            .expect("Failed to serialize");

        let mut file = File::create(path)
            .expect("Unable to create file");

        file.write_all(json.as_bytes())
            .expect("Unable to write data");
    }
}
