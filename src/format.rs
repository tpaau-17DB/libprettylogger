//! Contains `LogFormatter`, used to create formatted log messages from raw log
//! structs.

/// Contains `LogFormatter`, used to create formatted log messages from raw log
/// structs.
use serde::{Serialize, Deserialize};
use chrono::{Local, DateTime};

use crate::{
    LogType, Error,
    colors::{Color, color_text},
    config::LogStruct,
};

/// Formats raw log structs into log messages by applying both the log
/// message's configuration and the formatter's own settings.
///
/// # Examples
///
/// Using a `LogFormatter` to print a log:
/// ```
/// # use prettylogger::{
/// #    config::LogStruct,
/// #    format::LogFormatter,
/// # };
/// // Create a `LogFormatter` with default configuration
/// let mut formatter = LogFormatter::default();
///
/// // Set a log format
/// formatter.set_log_format("[ %h %m ]");
///
/// // Obtain a formatted log from a `LogStruct`
/// let log = formatter.format_log(&LogStruct::debug("Hello from LogStruct!"));
///
/// // Print the formatted log message
/// print!("{}", &log);
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize,
    Deserialize)]
pub struct LogFormatter {
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

    #[serde(skip)]
    pub(crate) show_datetime: Option<bool>,
}

impl LogFormatter {
    pub(crate) fn get_datetime_formatted(&mut self, datetime: &DateTime<Local>) -> String {
        match self.show_datetime {
            Some(b) => {
                match b {
                    true => {
                        return datetime.format(&self.datetime_format)
                            .to_string();
                    },
                    false => {
                        return String::new();
                    },
                }

            },
            None => {
                match self.log_format.contains("%d") {
                    true => {
                        self.show_datetime = Some(true);
                        return datetime.format(&self.datetime_format)
                            .to_string();
                    },
                    false => {
                        self.show_datetime = Some(false);
                        return String::new();
                    },
                }
            }
        }
    }

    pub(crate) fn log_header_color(&self, log_type: LogType) -> Color {
        match log_type {
            LogType::Debug => self.debug_color.clone(),
            LogType::Info => self.info_color.clone(),
            LogType::Warning => self.warning_color.clone(),
            LogType::Err => self.error_color.clone(),
            LogType::FatalError => self.fatal_color.clone(),
        }
    }

    pub(crate) fn colorify(&self, text: &str, color: Color) -> String {
        if self.log_header_color_enabled {
            return color_text(text, color);
        }
        return text.to_string()
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

    pub(crate) fn get_log_headers(&mut self, log: &LogStruct)
    -> (String, String) {
        let header = self.get_log_type_header(log.log_type);
        let datetime = self.get_datetime_formatted(&log.datetime);
        return (header, datetime);
    }

    /// Returns a log entry from a `LogStruct` based on current `LogFormatter`
    /// configuration.
    ///
    /// # Examples
    /// ```
    /// # use prettylogger::{format::LogFormatter, config::LogStruct};
    /// let mut formatter = LogFormatter::default();
    /// let log_string = formatter.format_log(&LogStruct::error("Error!"));
    /// ```
    pub fn format_log(&mut self, log: &LogStruct) -> String {
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
                            'm' => result += &log.message,
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

    /// Toggles colored log headers.
    /// * `true`: Log headers will have colors
    /// * `false`: No colors :(
    pub fn toggle_log_header_color<I: Into<bool>>(&mut self, enabled: I) {
        self.log_header_color_enabled = enabled.into();
    }

    /// Sets **debug log header** color.
    pub fn set_debug_color<I: Into<Color>>(&mut self, color: I) {
        self.debug_color = color.into();
    }

    /// Sets **info log header** color.
    pub fn set_info_color<I: Into<Color>>(&mut self, color: I) {
        self.info_color = color.into();
    }

    /// Sets **warning header** color.
    pub fn set_warning_color<I: Into<Color>>(&mut self, color: I) {
        self. warning_color = color.into();
    }

    /// Sets **error header** color.
    pub fn set_error_color<I: Into<Color>>(&mut self, color: I) {
        self.error_color = color.into();
    }

    /// Sets **fatal error header** color.
    pub fn set_fatal_color<I: Into<Color>>(&mut self, color: I) {
        self.fatal_color = color.into();
    }

    /// Sets **debug log header** format.
    pub fn set_debug_header(&mut self, header: &str) {
        self.debug_header = header.to_string();
    }

    /// Sets **info log header** format.
    pub fn set_info_header(&mut self, header: &str) {
        self.info_header = header.to_string();
    }

    /// Sets **warning header** format.
    pub fn set_warning_header(&mut self, header: &str) {
        self.warning_header = header.to_string();
    }

    /// Sets **error header** format.
    pub fn set_error_header(&mut self, header: &str) {
        self.error_header = header.to_string();
    }

    /// Sets **fatal error header** format.
    pub fn set_fatal_header(&mut self, header: &str) {
        self.fatal_header = header.to_string();
    }

    /// Sets datetime format.
    pub fn set_datetime_format(&mut self, format: &str) {
        self.datetime_format = String::from(format);
        self.show_datetime = None;
    }

    /// Sets the log format.
    ///
    /// Returns an error when the `%m` placeholder is missing.
    ///
    /// There are several placeholders in a log format string:
    /// * `%m`: The log message (this placeholder is mandatory, you will
    ///   get an error if you don't include it in your log format).
    /// * `%h`: The header indicating the log type (e.g., debug, error, etc.)
    /// * `%d`: The timestamp.
    ///
    /// You can have multiple placeholders of the same type in a format string.
    ///
    /// # Examples
    /// ```
    /// # use prettylogger::{
    /// #     format::LogFormatter,
    /// #     config::LogStruct,
    /// # };
    /// let mut formatter = LogFormatter::default();
    ///
    /// // Do a nice XML-like format
    /// formatter.set_log_format("<l> <h>%h</h> <m>%m</m> </l>");
    /// print!("{}", formatter.format_log(&LogStruct::debug("Hello, World!")));
    /// ```
    pub fn set_log_format(&mut self, format: &str) -> Result<(), Error> {
        if format.contains("%m") {
            self.log_format = String::from(format);
            Ok(())
        }
        else {
            Err(Error::new("Expected a message placeholder!"))
        }
    }
}

impl Default for LogFormatter {
    fn default() -> LogFormatter {
        let log_format = String::from("[%h] %m");
        LogFormatter {
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

            show_datetime: None,
        }
    }
}
