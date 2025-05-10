//! Highly customizable logger library.

/// Highly customizable logger library.
#[cfg(test)]
mod tests;

#[doc = include_str!("../README.md")]
mod fileio;
mod json;
mod getters;

pub mod colors;
pub mod config;
pub mod format;

use format::LogFormatter;
use serde::{Serialize, Deserialize};
use config::{Verbosity, LogStruct, LogType, OnDropPolicy};
use fileio::{append_to_file, ensure_writable_file_exists, expand_env_vars,
expand_tilde, overwrite_file};

/// The `Logger` struct used to print logs.
///
/// # Example
/// ```
/// # use prettylogger::Logger;
/// // Create a `Logger` with default configuration:
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

    pub(crate) console_out_enabled: bool,

    pub(crate) use_custom_log_buffer: bool,

    pub(crate) verbosity: Verbosity,
    pub(crate) filtering_enabled: bool,

    pub(crate) file_logging_enabled: bool,
    pub(crate) log_file_path: String,
    pub(crate) file_log_buffer_max_size: u32,
    pub(crate) on_drop_policy: OnDropPolicy,

    // Dynamic variables that shouldn't be included in the template file:
    #[serde(skip)]
    pub(crate) custom_log_buffer: Vec<LogStruct>,
    #[serde(skip)]
    pub(crate) file_log_buffer: Vec<LogStruct>,
    #[serde(skip)]
    pub(crate) show_datetime: bool,
    #[serde(skip)]
    pub(crate) log_file_lock: bool,
    #[serde(skip)]
    pub(crate) log_count: u128,
}

impl Logger {
    /// Returns true if log is to be filtered and false otherwise.
    pub(crate) fn filter_log(&self, log_type: LogType) -> bool {
        if self.filtering_enabled {
            return (log_type as i32) < self.verbosity as i32;
        }
        return false;
    }

    pub(crate) fn drop_flush(&mut self) {
        if self.file_logging_enabled {
            let _ = self.flush_file_log_buffer(true);
        }
    }

    pub(crate) fn flush_file_log_buffer(&mut self, is_drop_flush: bool)
    -> Result<(), Error> {
        if self.log_file_lock {
            if is_drop_flush {
                match self.on_drop_policy {
                    OnDropPolicy::IgnoreLogFileLock => { }
                    OnDropPolicy::DiscardLogBuffer => {
                        return Err(Error::new(&format!("Log file lock enabled and on drop policy set to {}!",
                            self.on_drop_policy)));
                    }
                }
            }
            else {
               return Err(Error::new(&"Log file lock is enabled!"))
            }
        }
        let mut buf = String::from("");

        for log in &self.file_log_buffer {
            buf += &self.formatter.format_log(log);
        }

        self.file_log_buffer = Vec::new();
        let result = append_to_file(&self.log_file_path, &buf);

        match result {
            Ok(_) => Ok(()),
            Err(_) => {
                self.file_logging_enabled = false;
                return Err(Error::new(&"Failed to write log buffer to a file!"))
            },
        }
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
        let log_str = self.formatter.format_log(log);

        if self.console_out_enabled {
            eprint!("{}", log_str);
        }

        if self.use_custom_log_buffer {
            self.custom_log_buffer.push(log.clone());
        }

        if self.file_logging_enabled {
            self.file_log_buffer.push(log.clone());

            if self.file_log_buffer_max_size != 0
            && self.file_log_buffer.len() >=
            self.file_log_buffer_max_size.try_into().unwrap() {
                let _ = self.flush_file_log_buffer(false);
            }
        }
    }

    /// Flushes log buffer (if file logging is enabled and log file lock
    /// disabled, it writes the log buffer to a file).
    ///
    /// Returns an error when there is an issue writing to a file or log
    /// file lock is enabled.
    pub fn flush(&mut self) -> Result<(), Error> {
        if self.file_logging_enabled {
            match self.flush_file_log_buffer(false) {
                Ok(_) => Ok(()),
                Err(e) => { 
                    return Err(Error::new(&e.to_string())); 
                }
            }
        }
        else {
            return Err(Error::new(&"File logging is disabled!"));
        }
    }

    /// Prints a **debug message** to `stdout`.
    pub fn debug(&mut self, message: &str) {
        if self.filter_log(LogType::Debug) {
            return;
        }
        let log = LogStruct::debug(message);
        self.print_log(&log);
    }

    /// Prints a **debug message** to `stdout`, bypasses filtering.
    pub fn debug_no_filtering(&mut self, message: &str) {
        let log = LogStruct::debug(message);
        self.print_log(&log);
    }

    /// Prints an **informational message** to `stdout`.
    pub fn info(&mut self, message: &str) {
        if self.filter_log(LogType::Info) {
            return;
        }
        let log = LogStruct::info(message);
        self.print_log(&log);
    }

    /// Prints an **informational message** to `stdout`, bypasses filtering.
    pub fn info_no_filtering(&mut self, message: &str) {
        let log = LogStruct::info(message);
        self.print_log(&log);
    }

    /// Prints a **warning** to `stdout`.
    pub fn warning(&mut self, message: &str) {
        if self.filter_log(LogType::Warning) {
            return;
        }
        let log = LogStruct::warning(message);
        self.print_log(&log);
    }

    /// Prints a **warning** to `stdout`, bypasses filtering.
    pub fn warning_no_filtering(&mut self, message: &str) {
        let log = LogStruct::warning(message);
        self.print_log(&log);
    }

    /// Prints an **error** to  `stderr`.
    pub fn error(&mut self, message: &str) {
        let log = LogStruct::error(message);
        self.print_log(&log);
    }

    /// Prints a **fatal error** to `stderr`.
    pub fn fatal(&mut self, message: &str) {
        let log = LogStruct::fatal_error(message);
        self.print_log(&log);
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

    /// Sets log file path.
    ///
    /// Returns an error if the path is inaccessible.
    ///
    /// https://github.com/tpaau-17DB/libprettylogger?tab=readme-ov-file#file-logging
    ///
    /// # Example
    /// ```
    /// # use prettylogger::Logger;
    /// # let mut logger = Logger::default();
    /// # let mut path = std::env::temp_dir();
    /// # path.push("libprettylogger-tests/set_log_file_path.log");
    /// # let path = &path.to_str().unwrap().to_string();
    /// # logger.set_log_file_path(path);
    /// // Set the log file path first:
    /// logger.set_log_file_path(path);
    /// // Then enable file logging:
    /// logger.toggle_file_logging(true);
    /// ```
    pub fn set_log_file_path(&mut self, path: &str) -> Result<(), Error> {
        let path: &str = &expand_env_vars(&expand_tilde(path));
        if ensure_writable_file_exists(path) {
            self.log_file_path = path.to_string();
            match overwrite_file(path, "") {
                Ok(_) => { Ok(()) },
                Err(e) => {
                    self.error(&format!("Failed to open file '{}' for writing. {}",
                        path, e.to_string()));
                    return Err(Error::new(&e.to_string()))
                }
            }
        }
        else {
            self.error(&format!("Failed to open file '{}' for writing. File not writeable", path));
            return Err(Error::new(&"File is not writable!"))
        }
    }

    /// Toggles file logging.
    ///
    /// Before enabling file logging, ensure that the log file path is set.
    /// This is because this method checks if the log file is writable. If
    /// the log file path is not set, or the file is not writable, enabling
    /// file logging will result in an error.
    ///
    /// # Example
    /// ```
    /// # use prettylogger::Logger;
    /// # let mut logger = Logger::default();
    /// # let mut path = std::env::temp_dir();
    /// # path.push("libprettylogger-tests/toggle_file_logging.log");
    /// # let path = &path.to_str().unwrap().to_string();
    /// # logger.set_log_file_path(path);
    /// // Set the log file path first:
    /// logger.set_log_file_path(path);
    /// // Then enable file logging:
    /// logger.toggle_file_logging(true);
    /// ```
    pub fn toggle_file_logging<I: Into<bool>>(&mut self, enabled: I)
   -> Result<(), std::io::Error> {
        if !enabled.into() {
            self.file_logging_enabled = false;
            Ok(())
        }
        else if ensure_writable_file_exists(&self.log_file_path) {
            self.file_logging_enabled = true;
            Ok(())
        }
        else {
            self.error(&format!("Failed to open file '{}' for writing!",
                self.log_file_path));
            Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied,
                "File is not writable!"))
        }
    }

    /// Sets the maximum allowed size for the log buffer.
    ///
    /// When the buffer exceeds its max size, it gets flushed
    /// automatically to the log file. When set to `0`,  automatic flushing is
    /// disabled and the buffer can only be flushed manually.
    ///
    /// If a log file lock is active, the log buffer will not be flushed
    /// automatically, regardless of the size limit.
    ///
    /// # Example
    /// ```
    /// # use prettylogger::Logger;
    /// # let mut path = std::env::temp_dir();
    /// # path.push("libprettylogger-tests/set_max_log_buffer_size.log");
    /// # let path = &path.to_str().unwrap().to_string();
    /// let mut logger = Logger::default();
    /// logger.set_log_file_path(path);
    /// logger.toggle_file_logging(true);
    ///
    /// // Make `Logger` flush the log buffer every 16 logs:
    /// logger.set_max_log_buffer_size(16 as u32);
    ///
    /// let mut i = 0;
    /// loop {
    ///     logger.info("Yay!");
    ///     i += 1;
    ///     if i >= 16 {
    ///         break;
    ///     }
    /// }
    /// // Here the buffer gets flushed, after sixteenth iteration.
    /// ```
    pub fn set_max_log_buffer_size<I: Into<u32>>(&mut self, size: I) {
        self.file_log_buffer_max_size = size.into();
    }

    /// Log file lock can be used to prevent race conditions when there are
    /// multiple threads accessing the log file at the same time.
    ///
    /// # WARNING: leaving this option on for a long period of time will
    /// cause high memory usage!
    ///
    /// * `true`: When log file lock is enabled, logger won't flush into the
    ///     log file. Instead, it will wait until the lock is disabled. You
    ///     will not loose any logs, they will be stored in the log buffer
    ///     even when it exceeds its size limit.
    /// * `false`: Logger will write to the log file normally.
    pub fn toggle_log_file_lock<I: Into<bool>>(&mut self, enabled: I) {
        self.log_file_lock = enabled.into();
    }

    /// Sets `Logger`'s on drop log file policy.
    ///
    /// # Example
    /// ```
    /// # use prettylogger::{Logger, config::OnDropPolicy};
    /// # let mut logger = Logger::default();
    /// logger.set_on_drop_file_policy(OnDropPolicy::IgnoreLogFileLock);
    /// ```
    pub fn set_on_drop_file_policy<I: Into<OnDropPolicy>>(&mut self, policy: I) {
        self.on_drop_policy = policy.into();
    }

    /// Toggles printing logs to `stdout`.
    /// * `true`: Logs will be printed in your terminal's `stdout`.
    /// * `false`: No log output in your terminal.
    pub fn toggle_console_output<I: Into<bool>>(&mut self, enabled: I) {
        self.console_out_enabled = enabled.into();
    }

    /// Toggles the usage of a custom log buffer.
    /// * `true`: Logs will be stored in a buffer inside `Logger` and can
    ///     be cloned using the `clone_log_buffer()` method. Be aware that
    ///     this will lead to high memory usage if turned on for a log
    ///     period of time.
    /// * `false`: Logs will not be stored in a log buffer.
    pub fn toggle_custom_log_buffer<I: Into<bool>>(&mut self, enabled: I) {
        self.use_custom_log_buffer = enabled.into();
    }

    /// Clears the custom log buffer.
    pub fn clear_log_buffer(&mut self) {
        self.custom_log_buffer = Vec::new();
    }
}

impl Default for Logger {
    fn default() -> Self {
        let log_format = String::from("[%h] %m");
        Logger {
            console_out_enabled: true,

            use_custom_log_buffer: false,

            verbosity: Verbosity::default(),
            filtering_enabled: true,

            formatter: LogFormatter::default(),

            file_logging_enabled: false,
            log_file_path: String::new(),
            file_log_buffer_max_size: 128,
            on_drop_policy: OnDropPolicy::default(),

            custom_log_buffer: Vec::new(),
            file_log_buffer: Vec::new(),
            show_datetime: log_format.contains("%d"),
            log_file_lock: false,
            log_count: 1,
        }
    }
    
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.drop_flush();
    }
}

/// Represents an error thrown by the Logger.
#[derive(Clone, Eq, PartialEq, Debug)]
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
