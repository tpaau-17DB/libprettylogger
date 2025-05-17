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

/// Logger capable of filtering logs, formatting them and distributing them to
/// various streams.
///
/// The `Logger` struct is modular and itself only filters the logs, relying
/// on `LogFormatter` and `LogOutput` for log formatting and outputting. 
///
/// Additionally, `Logger` includes a template system with built-in methods and
/// constructors for easy JSON serialization and deserialization.
///
/// # Examples
///
/// Creating a `Logger` struct and printing out some logs:
/// ```
/// # use prettylogger::Logger;
/// // Create a `Logger` instance with default configuration
/// let mut logger = Logger::default();
///
/// // Print log messages
/// logger.debug("debug message");
/// logger.info("info message");
/// logger.warning("warning message");
/// logger.error("error message");
/// logger.fatal("fatal error message");
/// ```
///
/// Configuring `Logger`s formatter:
/// ```
/// # use prettylogger::{
/// #     Logger,
/// #     colors::Color,
/// # };
/// // Create a `Logger` instance with default configuration
/// let mut logger = Logger::default();
///
/// // Set a simple log format
/// logger.formatter.set_log_format("[ %d ] %m");
///
/// // Change debug log header color
/// logger.formatter.set_debug_color(Color::Red);
///
/// // Set a fatal log header
/// logger.formatter.set_fatal_header("--FATAL--");
///
/// // Configure datetime format
/// logger.formatter.set_datetime_format("%H:%M");
/// ```
///
/// Enabling log buffering:
/// ```
/// # use prettylogger::{
/// #     Logger,
/// #     output::Toggleable,
/// # };
/// // Create a `Logger` instance with default configuration
/// let mut logger = Logger::default();
///
/// // Enable log buffering
/// logger.output.buffer_output.enable();
///
/// for i in 0..128 {
///     logger.error(&format!("Error number {}", i));
/// }
///
/// // Get a reference to the log buffer
/// let buffer = logger.output.buffer_output.get_log_buffer();
/// ```
/// 
/// Enabling file logging:
/// ```
/// # use prettylogger::{
/// #     Logger,
/// #     output::Toggleable,
/// #     format::LogFormatter,
/// #     config::LogStruct,
/// # };
/// # let mut path = std::env::temp_dir();
/// # path.push("libprettylogger-tests/readme-logger-saving.json");
/// # let path = &path.to_str().unwrap().to_string();
/// // Create a `Logger` instance with default configuration
/// let mut logger = Logger::default();
///
/// // Required by `FileStream` for parsing logs:
/// let mut formatter = LogFormatter::default();
///
/// // Set the log file path **first**
/// logger.output.file_output.set_log_file_path(&path)
///     .expect("Failed to set the log file path!");
///
/// // Enable the output
/// logger.output.file_output.enable().
///     expect("Failed to enable the output!");
///
/// // Write to the log file buffer
/// logger.output.file_output.out(&LogStruct::debug("Hello from file!"),
///     &mut formatter).expect("Failed to write to the buffer!");
///
/// // Flush the logs from the buffer to the log file
/// logger.output.file_output.flush();
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize,
    Deserialize)]
pub struct Logger {
    pub formatter: LogFormatter,
    pub output: LogOutput,

    pub(crate) verbosity: Verbosity,
    pub(crate) filtering_enabled: bool,
}

impl Logger {
    /// Returns true if log should be filtered and false otherwise.
    pub(crate) fn filter_log(&self, log_type: LogType) -> bool {
        if self.filtering_enabled {
            return (log_type as i32) < self.verbosity as i32;
        }
        return false;
    }

    /// Prints a **debug message**.
    pub fn debug(&mut self, message: &str) {
        if self.filter_log(LogType::Debug) {
            return;
        }
        let log = LogStruct::debug(message);
        self.output.out(&log, &mut self.formatter);
    }

    /// Prints an **informational message**.
    pub fn info(&mut self, message: &str) {
        if self.filter_log(LogType::Info) {
            return;
        }
        let log = LogStruct::info(message);
        self.output.out(&log, &mut self.formatter);
    }

    /// Prints a **warning**.
    pub fn warning(&mut self, message: &str) {
        if self.filter_log(LogType::Warning) {
            return;
        }
        let log = LogStruct::warning(message);
        self.output.out(&log, &mut self.formatter);
    }

    /// Prints an **error**.
    pub fn error(&mut self, message: &str) {
        let log = LogStruct::error(message);
        self.output.out(&log, &mut self.formatter);
    }

    /// Prints a **fatal error**.
    pub fn fatal(&mut self, message: &str) {
        let log = LogStruct::fatal_error(message);
        self.output.out(&log, &mut self.formatter);
    }

    /// Sets logger `verbosity`.
    ///
    /// # Examples
    ///
    /// Setting `Logger` verbosity:
    /// ```
    /// # use prettylogger::{Logger, config::Verbosity};
    /// # let mut logger = Logger::default();
    /// logger.set_verbosity(Verbosity::Quiet);
    /// ```
    pub fn set_verbosity<I: Into<Verbosity>>(&mut self, verbosity: I) {
        self.verbosity = verbosity.into();
    }

    /// Enables log filtering.
    pub fn enable_log_filtering(&mut self) {
        self.filtering_enabled = true;
    }

    /// Disables log filtering.
    pub fn disable_log_filtering(&mut self) {
        self.filtering_enabled = false;
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            output: LogOutput::default(),

            verbosity: Verbosity::default(),
            filtering_enabled: true,

            formatter: LogFormatter::default(),
        }
    }
    
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.output.file_output.drop_flush();
    }
}

/// Represents errors thrown by the `prettylogger` crate.
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
