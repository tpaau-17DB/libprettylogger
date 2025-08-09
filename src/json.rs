use std::{
    fs::{
        File,
        read_to_string
    },
    io::Write,
};

use crate::{
    Logger,
    Error
};

impl Logger {
    /// Creates a `Logger` instance from a JSON template as string.
    ///
    /// # Examples
    ///
    /// Deserializing `Logger` from a JSON string:
    /// ```
    /// # use prettylogger::Logger;
    /// let pretty_json = serde_json::to_string_pretty(&Logger::default())
    ///     .expect("Failed to serialize logger!");
    ///
    /// let mut logger = Logger::from_template_str(&pretty_json)
    ///     .expect("Failed to deserialize logger!");
    /// # let raw_json = serde_json::to_string(&Logger::default())
    /// #    .expect("Failed to serialize logger!");
    /// # assert_eq!(Logger::default(), Logger::from_template_str(&pretty_json)
    /// #    .expect("Failed to deserialize logger!"));
    /// # assert_eq!(Logger::default(), Logger::from_template_str(&raw_json)
    /// #    .expect("Failed to deserialize logger!"));
    /// ```
    pub fn from_template_str(template: &str) -> Result<Logger, Error> {
        let result: Result<Logger, serde_json::Error>
            = serde_json::from_str(template);
        match result {
            Ok(logger) => {
                Ok(logger)
            },
            Err(e) => Err(Error::new(&e.to_string()))
        }
    }

    /// Creates a `Logger` instance from a template file.
    ///
    /// # Examples
    ///
    /// Deserializing `Logger` from a JSON file:
    /// ```
    /// # use prettylogger::Logger;
    /// # let mut path = std::env::temp_dir();
    /// # path.push("libprettylogger-tests/from-template.json");
    /// # let path = &path.to_str().unwrap().to_string();
    /// # Logger::default().save_template(path);
    /// let mut logger = Logger::from_template(path)
    ///     .expect("Failed to deserialize logger!");
    /// # assert_eq!(Logger::default(), logger);
    /// ```
    pub fn from_template(path: &str) -> Result<Logger, Error> {
        match read_to_string(path) {
            Ok(contents) => {
                Logger::from_template_str(&contents)
            },
            Err(e) => Err(Error::new(&e.to_string()))
        }
    }

    /// Saves a `Logger` instance to template file.
    ///
    /// # Examples
    ///
    /// Serializing a `Logger` instance to a file:
    /// ```
    /// # use prettylogger::Logger;
    /// # let mut path = std::env::temp_dir();
    /// # path.push("libprettylogger-tests/from-template.json");
    /// # let path = &path.to_str().unwrap().to_string();
    /// let mut logger = Logger::default();
    /// logger.save_template(path);
    /// # assert_eq!(Logger::from_template(path)
    ///     .expect("Failed to deserialize logger!"), Logger::default());
    /// ```
    pub fn save_template(&self, path: &str) -> Result<(), Error> {
        let json = serde_json::to_string_pretty(self);
        match json {
            Ok(json) => {
                match File::create(path) {
                    Ok(mut file) => {
                        match file.write_all(json.as_bytes()) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                Err(Error::new(&e.to_string()))
                            }
                        }
                    },
                    Err(e) => Err(Error::new(&e.to_string()))
                }
            },
            Err(e) => Err(Error::new(&e.to_string()))
        }
    }
}
