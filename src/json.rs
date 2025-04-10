use crate::*;
use std::{
    fs::*,
    io::Write,
};
use crate::fileio::{expand_env_vars, expand_tilde};

impl Logger {
    /// Creates a `Logger` instance from a JSON template as string.
    ///
    /// https://github.com/tpaau-17DB/libprettylogger?tab=readme-ov-file#logger-templates
    ///
    /// # Example
    /// ```
    /// # use prettylogger::Logger;
    /// let pretty_json = serde_json::to_string_pretty(&Logger::default())
    ///     .expect("Failed to serialize logger!");
    /// let raw_json = serde_json::to_string(&Logger::default())
    ///     .expect("Failed to serialize logger!");
    /// assert_eq!(Logger::default(), Logger::from_template_str(&pretty_json)
    ///     .expect("Failed to deserialize logger!"));
    /// assert_eq!(Logger::default(), Logger::from_template_str(&raw_json)
    ///     .expect("Failed to deserialize logger!"));
    /// ```
    pub fn from_template_str(template: &str) -> Result<Logger, String> {
        let result: Result<Logger, serde_json::Error>
            = serde_json::from_str(&template);
        match result {
            Ok(mut logger) => {
                logger.log_count += 1;
                logger.show_datetime = logger.log_format.contains("%d");

                return Ok(logger);
            },
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    /// Creates a `Logger` instance from a template file.
    ///
    /// Automatically expands environment variables.
    ///
    /// https://github.com/tpaau-17DB/libprettylogger?tab=readme-ov-file#logger-templates
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
    pub fn from_template(path: &str) -> Result<Logger, String> {
        let path = expand_env_vars(&expand_tilde(path));

        match read_to_string(path) {
            Ok(contents) => {
                return Logger::from_template_str(&contents);
            },
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    /// Saves a `Logger` to template file.
    ///
    /// Automatically expands environment variables.
    ///
    /// https://github.com/tpaau-17DB/libprettylogger?tab=readme-ov-file#logger-templates
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
    pub fn save_template(&self, path: &str) -> Result<(), String> {
        let path = expand_env_vars(&expand_tilde(path));

        let json: Result<_, serde_json::Error>
            = serde_json::to_string_pretty(self);

        match json {
            Ok(json) => {
                match File::create(path) {
                    Ok(mut file) => {
                        match file.write_all(json.as_bytes()) {
                            Ok(_) => {
                                return Ok(());
                            },
                            Err(e) => {
                                return Err(e.to_string());
                            }
                        }
                    },
                    Err(e) => {
                        return Err(e.to_string());
                    }
                }
            },
            Err(e) => {
                return Err(e.to_string())
            }
        }
    }
}
