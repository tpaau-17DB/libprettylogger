//! Highly customizable logger library.

/// Highly customizable logger library.
#[cfg(test)]
mod tests;

#[doc = include_str!("../README.md")]
mod fileio;
mod json;

pub mod colors;
pub mod config;
pub mod format;
pub mod output;

use format::LogFormatter;
use serde::{Serialize, Deserialize};
use config::{Verbosity, LogStruct, LogType};
use output::LogOutput;

/// The `Logger` struct used to print logs.
///
/// # Example
/// ```
/// # use prettylogger::Logger;
/// // Create a `Logger` with default configuration
/// let mut logger = Logger::default();
/// logger.debug("debug message");
/// logger.info("info message");
/// logger.warning("warning message");
/// logger.error("error message");
/// logger.fatal("fatal error message");
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize,
    Deserialize)]
pub struct Logger {
    pub formatter: LogFormatter,
    pub output: LogOutput,

    pub(crate) verbosity: Verbosity,
    pub(crate) filtering_enabled: bool,

    #[serde(skip)]
    pub (crate) log_count: u64,
}

impl Logger {
    /// Returns true if log is to be filtered and false otherwise.
    pub(crate) fn filter_log(&self, log_type: LogType) -> bool {
        if self.filtering_enabled {
            return (log_type as i32) < self.verbosity as i32;
        }
        return false;
    }

    /// Used to print a log from a `LogStruct`.
    ///
    /// Bypasses log filtering.
    ///
    /// # Example:
    /// ```
    /// # use prettylogger::{Logger, config::LogStruct};
    /// # let mut logger = Logger::default();
    /// logger.print_log(&LogStruct::error("&%$#@!"));
    /// ```
    pub fn print_log(&mut self, log: &LogStruct) {
        self.log_count += 1;
        self.output.out(log, &self.formatter);
    }

    /// Prints a **debug message**.
    pub fn debug(&mut self, message: &str) {
        if self.filter_log(LogType::Debug) {
            return;
        }
        let log = LogStruct::debug(message);
        self.output.out(&log, &self.formatter);
    }

    /// Prints an **informational message**.
    pub fn info(&mut self, message: &str) {
        if self.filter_log(LogType::Info) {
            return;
        }
        let log = LogStruct::info(message);
        self.output.out(&log, &self.formatter);
    }

    /// Prints a **warning**.
    pub fn warning(&mut self, message: &str) {
        if self.filter_log(LogType::Warning) {
            return;
        }
        let log = LogStruct::warning(message);
        self.output.out(&log, &self.formatter);
    }

    /// Prints an **error**.
    pub fn error(&mut self, message: &str) {
        let log = LogStruct::error(message);
        self.output.out(&log, &self.formatter);
    }

    /// Prints a **fatal error**.
    pub fn fatal(&mut self, message: &str) {
        let log = LogStruct::fatal_error(message);
        self.output.out(&log, &self.formatter);
    }

    /// Sets logger `verbosity`.
    ///
    /// # Example
    /// ```
    /// # use prettylogger::{Logger, config::Verbosity};
    /// # let mut logger = Logger::default();
    /// logger.set_verbosity(Verbosity::Quiet);
    /// ```
    pub fn set_verbosity<I: Into<Verbosity>>(&mut self, verbosity: I) {
        self.verbosity = verbosity.into();
    }

    /// Toggles log filtering.
    /// * **true**: logs will get filtered based on verbosity
    /// * **false**: log filtering will be disabled globally
    pub fn toggle_log_filtering<I: Into<bool>>(&mut self, enabled: I) {
        self.filtering_enabled = enabled.into();
    }

    pub fn get_log_count(&self) -> &u64 {
        return &self.log_count;
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            output: LogOutput::default(),

            verbosity: Verbosity::default(),
            filtering_enabled: true,

            formatter: LogFormatter::default(),
            log_count: 1,
        }
    }
    
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.output.file_output.drop_flush();
    }
}

/// Represents an error thrown by the Logger.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Error {
            message: msg.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error { }
