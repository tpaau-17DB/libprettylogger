//! Fancy logger library.

/// Fancy logger library.
#[cfg(test)]
mod tests;

#[doc = include_str!("../README.md")]
mod fileio;
mod json;

pub mod colors;
pub mod config;
pub mod format;
pub mod output;
pub mod glob;

use std::sync::Mutex;

use format::LogFormatter;
use serde::{
    Serialize,
    Deserialize
};
use config::{
    Verbosity,
    LogStruct,
    LogType
};
use output::LogOutput;

/// `Logger` capable of filtering logs, formatting them and distributing them
/// to various streams.
///
/// The `Logger` struct is modular and itself only filters the logs, relying
/// on `LogFormatter` and `LogOutput` for log formatting and outputting.
///
/// Additionally, `Logger` includes a template system with built-in methods and
/// constructors for easy JSON serialization and deserialization.
///
/// # Examples
///
/// Using global logging:
/// ```
/// use prettylogger::{debug, info, warn, err, fatal};
///
/// let mut message = "debug";
/// debug!("Hello, this is a {message} log!");
///
/// message = "info";
/// info!("Hello, this is an {message} log!");
///
/// message = "warning";
/// warn!("Hello, this is a {message}!");
///
/// message = "error";
/// err!("Hello, this is an {message}!");
///
/// message = "fatal error";
/// fatal!("Hello, this is a {message}!")
/// ```
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
/// Configuring the formatter of a `Logger`:
/// ```
/// # use prettylogger::{
/// #     Logger,
/// #     colors::Color,
/// # };
/// // Create a `Logger` instance with default configuration
/// let mut logger = Logger::default();
///
/// // Set a simple log format
/// logger.formatter.lock().unwrap().set_log_format("[ %d ] %m");
///
/// // Change debug log header color
/// logger.formatter.lock().unwrap().set_debug_color(Color::Red);
///
/// // Set a fatal log header
/// logger.formatter.lock().unwrap().set_fatal_header("--FATAL--");
///
/// // Configure datetime format
/// logger.formatter.lock().unwrap().set_datetime_format("%H:%M");
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
/// logger.output.buffer_output.lock().unwrap().enable();
///
/// // Write to the log buffer 128 times
/// for i in 0..128 {
///     logger.error(&format!("Error number {}", i));
/// }
///
/// // Get a reference to the log buffer
/// let buffer = logger.output.buffer_output.lock().unwrap().get_log_buffer();
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
/// // Lock the file output and obtain a reference to it
/// let mut file_output = logger.output.file_output.lock().unwrap();
///
/// // Required by `FileStream` for parsing logs
/// let mut formatter = LogFormatter::default();
///
/// // Set the log file path **first**
/// file_output.set_log_file_path(&path)
///     .expect("Failed to set the log file path!");
///
/// // Enable the output
/// file_output.enable().
///     expect("Failed to enable the output!");
///
/// // Write to the log file buffer
/// file_output.out(&LogStruct::debug("Hello from file!"),
///     &mut formatter).expect("Failed to write to the buffer!");
///
/// // Flush the logs from the buffer to the log file
/// file_output.flush();
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Logger {
    pub formatter: Mutex<LogFormatter>,
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
        false
    }

    /// Prints a **debug message**.
    pub fn debug(&self, message: &str) {
        if self.filter_log(LogType::Debug) {
            return;
        }
        let log = LogStruct::debug(message);
        self.output.out(&log, &mut self.formatter.lock().unwrap());
    }

    /// Prints an **informational message**.
    pub fn info(&self, message: &str) {
        if self.filter_log(LogType::Info) {
            return;
        }
        let log = LogStruct::info(message);
        self.output.out(&log, &mut self.formatter.lock().unwrap());
    }

    /// Prints a **warning**.
    pub fn warning(&self, message: &str) {
        if self.filter_log(LogType::Warning) {
            return;
        }
        let log = LogStruct::warning(message);
        self.output.out(&log, &mut self.formatter.lock().unwrap());
    }

    /// Prints an **error**.
    pub fn error(&self, message: &str) {
        let log = LogStruct::error(message);
        self.output.out(&log, &mut self.formatter.lock().unwrap());
    }

    /// Prints a **fatal error**.
    pub fn fatal(&self, message: &str) {
        let log = LogStruct::fatal_error(message);
        self.output.out(&log, &mut self.formatter.lock().unwrap());
    }

    /// Sets `Logger` verbosity.
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

            formatter: LogFormatter::default().into(),
        }
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.output.file_output.lock().unwrap().drop_flush();
    }
}

impl PartialEq for Logger {
    fn eq(&self, other: &Self) -> bool {
        self.output == other.output &&
        self.verbosity == other.verbosity &&
        self.filtering_enabled == other.filtering_enabled
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
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error { }
