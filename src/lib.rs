//! A highly customizable logger library.

/// A highly customizable logger library.
#[cfg(test)]
mod tests;

#[doc = include_str!("../README.md")]
mod fileio;
mod setters;
mod json;

pub mod colors;
pub mod config;

use fileio::append_to_file;
use chrono::{Local, DateTime};
use serde::{Serialize, Deserialize};
use colors::{Color, color_text};
use config::{Verbosity, LogStruct, LogType, OnDropPolicy};

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
    pub(crate) console_out_enabled: bool,

    pub(crate) use_custom_log_buffer: bool,

    pub(crate) verbosity: Verbosity,
    pub(crate) filtering_enabled: bool,
    pub(crate) log_header_color_enabled: bool,

    pub(crate) debug_color: Color,
    pub(crate) info_color: Color,
    pub(crate) warning_color: Color,
    pub(crate) error_color: Color,
    pub(crate) fatal_color: Color,

    pub(crate) debug_header: String,
    pub(crate) info_header: String,
    pub(crate) warning_header: String,
    pub(crate) error_header: String,
    pub(crate) fatal_header: String,

    pub(crate) log_format: String,
    pub(crate) datetime_format: String,

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
    pub(crate) fn get_log_headers(&self, log: &LogStruct)
    -> (String, String, String) {
        let header = self.get_log_type_header(log.log_type);
        let datetime = self.get_datetime_formatted(&log.datetime);
        return (header, datetime, log.message.clone())
    }

    pub(crate) fn get_log_type_header(&self, log_type: LogType) -> String {
        match log_type {
            LogType::Debug => {
                return self.colorify(&self.debug_header,
                    self.log_header_color(log_type))
            }
            LogType::Info => {
                return self.colorify(&self.info_header,
                    self.log_header_color(log_type))
            }
            LogType::Warning => {
                return self.colorify(&self.warning_header,
                    self.log_header_color(log_type))
            }
            LogType::Err => {
                return self.colorify(&self.error_header,
                    self.log_header_color(log_type))
            }
            LogType::FatalError => {
                return self.colorify(&self.fatal_header,
                    self.log_header_color(log_type))
            }
        }
    }

    pub(crate) fn get_datetime_formatted(&self,
    datetime: &DateTime<Local>) -> String {
        if self.show_datetime {
            let datetime_formatted = datetime.format(&self.datetime_format);
            return datetime_formatted.to_string()
        }
        return String::new();
    }

    pub(crate) fn colorify(&self, text: &str, color: Color) -> String {
        if self.log_header_color_enabled {
            return color_text(text, color);
        }
        return text.to_string()
    }

    pub(crate) fn filter_log(&self, log_type: LogType) -> bool {
        if self.filtering_enabled {
            return (log_type as i32) < self.verbosity as i32;
        }
        return false;
    }

    pub(crate) fn log_header_color(&self, log_type: LogType) -> Color {
        match log_type {
            LogType::Debug => self.debug_color,
            LogType::Info => self.info_color,
            LogType::Warning => self.warning_color,
            LogType::Err => self.error_color,
            LogType::FatalError => self.fatal_color,
        }
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
                        let message = format!("Log file lock enabled and on
                            drop policy set to {}!",
                            self.on_drop_policy);
                        return Err(Error::new(&message));
                    }
                }
            }
            else {
               return Err(Error::new(&"Log file lock enabled!"))
            }
        }
        let mut buf = String::from("");

        for log in &self.file_log_buffer {
            buf += &self.format_log(log);
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
    /// # Example:
    /// ```
    /// # use prettylogger::{Logger, config::LogStruct};
    /// # let mut logger = Logger::default();
    /// logger.print_log(&LogStruct::error("&%$#@!"));
    /// ```
    pub fn print_log(&mut self, log: &LogStruct) {
        self.log_count += 1;
        let log_str = self.format_log(log);

        if self.console_out_enabled {
            if log.log_type == LogType::Warning
            || log.log_type == LogType::Err
            || log.log_type == LogType::FatalError {
                eprint!("{}", log_str);
            }
            else {
                print!("{}", log_str);
            }
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

    /// Returns a log entry from a `LogStruct` based on current `Logger`
    /// configuration.
    ///
    /// # Example:
    /// ```
    /// # use prettylogger::{Logger, config::LogStruct};
    /// # let mut logger = Logger::default();
    /// let log_string = logger.format_log(&LogStruct::error("ZXJyb3IK"));
    /// ```
    pub fn format_log(&self, log: &LogStruct) -> String {
        let headers = self.get_log_headers(log);
        let mut result = String::new();
        let mut char_iter = self
            .log_format.char_indices().peekable();

        while let Some((_, c)) = char_iter.next() {
            match c {
                '%' => {
                    if let Some((_, nc)) = char_iter.peek() {
                        match nc {
                            'h' => result += &headers.0,
                            'd' => result += &headers.1,
                            'm' => result += &headers.2,
                            'c' => result += &self.log_count.to_string(),
                            _ => result += &nc.to_string(),
                        }
                        char_iter.next();
                    }
                }
                _ => {
                    result += &c.to_string();
                }
            }
        }

        result += "\n";
        return result
    }

    /// Flushes log buffer (if file logging is enabled and log file lock
    /// disabled, it writes the log buffer to a file).
    ///
    /// Returns an error when there is an issue writing to a file or log
    /// file lock is enabled.
    pub fn flush(&mut self) -> Result<(), Error> {
        if self.file_logging_enabled {
            self.flush_file_log_buffer(false)?;
        }
        return Ok(());
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

    /// Returns a reference to the custom log buffer.
    pub fn log_buffer(&self) -> &Vec<LogStruct> {
        return &self.custom_log_buffer;
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
            log_header_color_enabled: true,

            debug_color: Color::Blue,
            info_color: Color::Green,
            warning_color: Color::Yellow,
            error_color: Color::Red,
            fatal_color: Color::Magenta,

            debug_header: String::from("DBG"),
            info_header: String::from("INF"),
            warning_header: String::from("WAR"),
            error_header: String::from("ERR"),
            fatal_header: String::from("FATAL"),

            log_format: log_format.clone(),
            datetime_format: String::from("%Y-%m-%d %H:%M:%S"),

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
