/// Holds implementation of the most crucial structs and enums of the entire
/// library.
///
/// # Structs:
/// `Logger` -> The core of the library.
/// `LogStruct` -> Representation of a single log message.
///
/// # Enums:
/// `OnDropPolicy` -> Used to set the on drop policy of the `Logger` (see https://github.com/tpaau-17DB/libprettylogger?tab=readme-ov-file#file-logging).
/// /// `LogType` -> Represents various types of logs, such as debug, info,
/// warning, etc.

use crate::{
    colors::*, fileio::append_to_file, filtering::*
};
use chrono::{Local, DateTime};
use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default,
    Serialize, Deserialize)]
/// Defines the policy for handling log file flushing when the logger is
/// dropped.
///
/// The available options are:
/// - `IgnoreLogFileLock`: Completely ignores any log file lock and forces the
/// log entries to be written to the file regardless of the lock status.
/// - `DiscardLogBuffer`: If a log file lock is enabled, the log buffer is
/// discarded instead of writing to the file. This prevents race conditions.
///
/// The default policy is `DiscardLogBuffer`.
pub enum OnDropPolicy {
    /// Completely ignore log file lock and write to file anyway.
    IgnoreLogFileLock,
    #[default]
    /// Don't write to the log file when lock is enabled.
    DiscardLogBuffer,
}

impl Display for OnDropPolicy {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let level_str = match *self {
            OnDropPolicy::IgnoreLogFileLock => "IgnoreLogFileLock",
            OnDropPolicy::DiscardLogBuffer => "DiscardLogBuffer",
        };
        write!(f, "{}", level_str)
    }
}


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default,
    Serialize, Deserialize)]
/// Represents the different types of log messages.
///
/// This enum is used to categorize the severity or type of a log message.
/// The variants correspond to different levels of logging, from debugging
/// information to fatal errors.
///
/// The variants are:
/// * `Debug`: Represents debug-level log messages, typically used for
/// detailed internal information during development.
/// * `Info`: Represents informational log messages.
/// * `Warning`: Represents warning messages.
/// * `Err`: Represents error messages.
/// * `FatalError`: Represents critical errors that usually lead to program
/// termination or an unrecoverable state.
///
/// The default variant is `Info`.
pub enum LogType {
    Debug = 0,
    #[default]
    Info = 1,
    Warning = 2,
    Err = 3,
    FatalError = 4,
}

impl TryFrom<i32> for LogType {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LogType::Debug),
            1 => Ok(LogType::Info),
            2 => Ok(LogType::Warning),
            3 => Ok(LogType::Err),
            4 => Ok(LogType::FatalError),
            _ => Err("Invalid value! Please provide a value in range 0-9."),
        }
    }
}

impl Display for LogType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let level_str = match *self {
            LogType::Debug => "Debug",
            LogType::Info => "Info",
            LogType::Warning => "Warning",
            LogType::Err => "Error",
            LogType::FatalError => "FatalError",
        };
        write!(f, "{}", level_str)
    }
}

impl AsRef<str> for LogType {
    fn as_ref(&self) -> &str {
        match self {
            LogType::Debug => "Debug",
            LogType::Info => "Info",
            LogType::Warning => "Warning",
            LogType::Err => "Err",
            LogType::FatalError => "Fatal Error",
        }
    }
}


#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
/// Represents a single log entry.
///
/// This struct is used to store information about a single log message.
/// It includes the log's message, its type (e.g.,
/// `Debug`, `Error`, etc.), and the date and time when the log was created.
/// It can be used for storing logs in memory more efficiently.
///
/// Fields:
/// - `message`: The actual log message as a string.
/// - `log_type`: The type of the log (e.g., `Debug`, `Error`, `Info`, etc.).
/// - `datetime`: The timestamp of when the log entry was created.
pub struct LogStruct {
    pub message: String,
    pub log_type: LogType,
    /// The date and time at which the log was created.
    pub datetime: DateTime<Local>,
}

impl Display for LogStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Log: {}\nType: {:?}\nDateTime: {}",
            self.message,
            self.log_type,
            self.datetime
        )
    }
}


/// A logger struct used for printing logs.
///
/// # You can create a `Logger` instance with the default configuration using:
/// ```
/// # use prettylogger::logging::Logger;
/// let l = Logger::default();
/// ```
///
/// # Alternatively, initialize a `Logger` instance from a JSON template file:
/// ```ignore
/// Logger::from_template("/path/to/file.json");
/// ```
///
/// # Once you have a `Logger` instance, you can start printing logs:
/// ```
/// # use prettylogger::logging::Logger;
/// # let mut l = Logger::default();
/// l.debug("debug message");
/// l.info("info message");
/// l.warning("warning message");
/// l.error("error message");
/// l.fatal("fatal error message");
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default,
    Serialize, Deserialize)]
pub struct Logger {
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
    pub(crate) log_buffer_max_size: usize,
    pub(crate) on_drop_policy: OnDropPolicy,

    // Dynamic variables that shouldn't be included in the template file:
    #[serde(skip)]
    pub(crate) log_buffer: Vec<LogStruct>,
    #[serde(skip)]
    pub(crate) show_datetime: bool,
    #[serde(skip)]
    pub(crate) log_file_lock: bool,
}

impl Drop for Logger {
    fn drop(&mut self) {
        let _ = self.drop_flush();
    }
}

impl Logger {
    // INTERNAL METHODS

    pub(crate) fn print_log(&mut self, log: &LogStruct) {
        let log_str = self.format_log(log);

        if self.file_logging_enabled {
            self.log_buffer.push(log.clone());

            if self.log_buffer_max_size != 0
            && self.log_buffer.len() >= self.log_buffer_max_size {
                let _ = self.flush_file_log_buffer(false);
            }
        }

        print!("{}", log_str);
    }

    pub(crate) fn get_log_headers(&self, log: &LogStruct)
    -> (String, String, String) {
        let header = self.get_main_header(log.log_type);
        let datetime = self.get_datetime_formatted(&log.datetime);
        return (header, datetime, log.message.clone());
    }

    pub(crate) fn get_main_header(&self, log_type: LogType) -> String {
        match log_type {
            LogType::Debug => {
                self.colorify(&self.debug_header,
                    self.log_header_color(log_type))
            }
            LogType::Info => {
                self.colorify(&self.info_header,
                    self.log_header_color(log_type))
            }
            LogType::Warning => {
                self.colorify(&self.warning_header,
                    self.log_header_color(log_type))
            }
            LogType::Err => {
                self.colorify(&self.error_header,
                    self.log_header_color(log_type))
            }
            LogType::FatalError => {
                self.colorify(&self.fatal_header,
                    self.log_header_color(log_type))
            }
        }
    }

    pub(crate) fn get_datetime_formatted(&self,
    datetime: &DateTime<Local>) -> String {
        if self.show_datetime {
            let datetime_formatted = datetime.format(&self.datetime_format);
            return datetime_formatted.to_string();
        }
        else {
            return String::from("");
        }
    }

    pub(crate) fn colorify(&self, text: &str, color: Color) -> String {
        if self.log_header_color_enabled {
            if color != Color::None {
                return get_color_code(color) + text + &RESET;
            }
            else {
                return text.to_string();
            }
        }
        else {
            return text.to_string();
        }
    }

    pub(crate) fn filter_log(&self, log_type: LogType) -> bool {
        return !self.filtering_enabled
            || ((log_type as i32) < self.verbosity as i32)
    }

    pub(crate) fn get_datetime(&self) -> DateTime<Local> {
        return Local::now();
    }

    pub(crate) fn log_header_color(&self, log_type: LogType) -> Color {
        match log_type {
            LogType::Debug => { self.debug_color.clone() }
            LogType::Info => { self.info_color.clone() }
            LogType::Warning => { self.warning_color.clone() }
            LogType::Err => { self.error_color.clone() }
            LogType::FatalError => { self.fatal_color.clone() }
        }
    }

    pub(crate) fn drop_flush(&mut self) {
        if self.file_logging_enabled {
            let _ = self.flush_file_log_buffer(true);
        }
    }

    pub(crate) fn flush_file_log_buffer(&mut self, is_drop_flush: bool)
    -> Result<(), String> {
        if self.log_file_lock {
            if is_drop_flush {
                match self.on_drop_policy {
                    OnDropPolicy::IgnoreLogFileLock => { }
                    OnDropPolicy::DiscardLogBuffer => {
                        let message = format!("Log file lock enabled and on
                            drop policy set to {}!",
                            self.on_drop_policy);
                        return Err(message);
                    }
                }
            }
            else {
               return Err(String::from("Log file lock enabled!"))
            }
        }
        let mut buf = String::from("");

        for log in &self.log_buffer {
            buf += &self.format_log(&log);
        }

        self.log_buffer = Vec::new();
        let result = append_to_file(&self.log_file_path, &buf);

        match result {
            Ok(_) => Ok(()),
            Err(_) => {
                self.file_logging_enabled = false;
                Err(String::from("Failed to write log buffer to a file!"))
            },
        }
    }

    // CONSTRUCTORS

    /// Returns a `Logger` instance with default configuration applied.
    pub fn default() -> Self {
        Logger {
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

            log_format: String::from("[%h] %m"),
            datetime_format: String::from("%Y-%m-%d %H:%M:%S"),

            file_logging_enabled: false,
            log_file_path: String::new(),
            log_buffer_max_size: 128,
            on_drop_policy: OnDropPolicy::default(),

            show_datetime: false,
            log_buffer: Vec::new(),
            log_file_lock: false,
        }
    }


    // PUBLIC METHODS

    /// Returns a log entry out of a `LogStruct` based on current `Logger`
    /// configuration.
    pub fn format_log(&self, log: &LogStruct) -> String {
        let headers = self.get_log_headers(&log);
        let mut result = String::new();
        let mut char_iter = self.log_format.char_indices().peekable();

        while let Some((_, c)) = char_iter.next() {
            match c {
                '%' => {
                    if let Some((_, nc)) = char_iter.peek() {
                        match nc {
                            'h' => {
                                result += &headers.0;
                                char_iter.next();
                            }
                            'd' => {
                                result += &headers.1;
                                char_iter.next();
                            }
                            'm' => {
                                result += &headers.2;
                                char_iter.next();
                            }
                            _ => {
                                result += &nc.to_string();
                                char_iter.next();
                            }
                        }
                    }
                }
                _ => {
                    result += &c.to_string();
                }
            }
        }

        result += &"\n";
        return result;
    }

    /// Flushes log buffer (if file logging is enabled and log file lock
    /// disabled, it writes the log buffer to a file) and then clears the log
    /// buffer.
    ///
    /// Returns an error when there is an issue writing to a file or log file
    /// lock is enabled.
    pub fn flush(&mut self) -> Result<(), String> {
        if self.file_logging_enabled {
            self.flush_file_log_buffer(false)?;
        }
        return Ok(());
    }

    /// Prints a **debug message**.
    pub fn debug(&mut self, message: &str) {
        if self.filter_log(LogType::Debug)
        {
            return;
        }
        let log = LogStruct {
            message: message.to_string(),
            log_type: LogType::Debug,
            datetime: self.get_datetime(),
        };
        self.print_log(&log);
    }

    /// Prints a **debug message**, bypasses filtering.
    pub fn debug_no_filtering(&mut self, message: &str) {
        let log = LogStruct {
            message: message.to_string(),
            log_type: LogType::Debug,
            datetime: self.get_datetime(),
        };
        self.print_log(&log);
    }

    /// Prints **info message**.
    pub fn info(&mut self, message: &str) {
        if self.filter_log(LogType::Info)
        {
            return;
        }
        let log = LogStruct {
            message: message.to_string(),
            log_type: LogType::Info,
            datetime: self.get_datetime(),
        };
        self.print_log(&log);
    }

    /// Prints **info message**, bypasses filtering.
    pub fn info_no_filtering(&mut self, message: &str) {
        let log = LogStruct {
            message: message.to_string(),
            log_type: LogType::Info,
            datetime: self.get_datetime(),
        };
        self.print_log(&log);
    }

    /// Prints a **warning**.
    pub fn warning(&mut self, message: &str) {
        if self.filter_log(LogType::Warning)
        {
            return;
        }
        let log = LogStruct {
            message: message.to_string(),
            log_type: LogType::Warning,
            datetime: self.get_datetime(),
        };
        self.print_log(&log);
    }

    /// Prints a **warning**, bypasses filtering.
    pub fn warning_no_filtering(&mut self, message: &str) {
        let log = LogStruct {
            message: message.to_string(),
            log_type: LogType::Warning,
            datetime: self.get_datetime(),
        };
        self.print_log(&log);
    }

    /// Prints an **error** (errors cannot be suppressed).
    pub fn error(&mut self, message: &str) {
        let log = LogStruct {
            message: message.to_string(),
            log_type: LogType::Err,
            datetime: self.get_datetime(),
        };
        self.print_log(&log);
    }

    /// Prints a **fatal error** (errors cannot be suppressed).
    pub fn fatal(&mut self, message: &str) {
        let log = LogStruct {
            message: message.to_string(),
            log_type: LogType::FatalError,
            datetime: self.get_datetime(),
        };
        self.print_log(&log);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env, io, path::PathBuf
    };

    fn get_current_dir() -> io::Result<PathBuf> {
        let current_dir = env::current_dir()?;
        Ok(current_dir)
    }

    #[test]
    fn test_log_filtering() {
        let mut l = Logger::default();
        l.toggle_log_filtering(true);
        l.set_verbosity(Verbosity::ErrorsOnly);

        if !l.filter_log(LogType::Debug) {
            panic!("A debug log should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
        }
        if !l.filter_log(LogType::Info) {
            panic!("An informative log should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
        }
        if !l.filter_log(LogType::Warning) {
            panic!("A warning log should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
        }

        l.set_verbosity(Verbosity::Quiet);
        if !l.filter_log(LogType::Debug) {
            panic!("A debug log should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
        }
        if !l.filter_log(LogType::Info) {
            panic!("An informative log should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
        }
        if l.filter_log(LogType::Warning) {
            panic!("A warning log not should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
        }

        l.set_verbosity(Verbosity::Standard);
        if !l.filter_log(LogType::Debug) {
            panic!("A debug log should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
        }
        if l.filter_log(LogType::Info) {
            panic!("An informative log should not get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
        }
        if l.filter_log(LogType::Warning) {
            panic!("A warning log not should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
        }

        l.set_verbosity(Verbosity::All);
        if l.filter_log(LogType::Debug) {
            panic!("A debug log should not get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
        }
        if l.filter_log(LogType::Info) {
            panic!("An informative log should not get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
        }
        if l.filter_log(LogType::Warning) {
            panic!("A warning log not should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
        }

        l.set_verbosity(Verbosity::All);
        l.toggle_log_filtering(true);
        if l.filter_log(LogType::Debug) {
            panic!("A debug log should not get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
        }
        if l.filter_log(LogType::Info) {
            panic!("An informative log should not get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
        }
        if l.filter_log(LogType::Warning) {
            panic!("A warning log not should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
        }
    }

    #[test]
    fn test_log_headers() {
        // Test if header format setting works
        let header = "askljdfh";

        let mut l = Logger::default();

        l.set_debug_header(header);
        if l.get_main_header(LogType::Debug) !=
        l.colorify(header, l.log_header_color(LogType::Debug)) {
            panic!("Debug headers do not match!");
        }
        l.set_info_header(header);
        if l.get_main_header(LogType::Info) !=
        l.colorify(header, l.log_header_color(LogType::Info)) {
            panic!("Info headers do not match!");
        }
        l.set_warning_header(header);
        if l.get_main_header(LogType::Warning) !=
        l.colorify(header, l.log_header_color(LogType::Warning)) {
            panic!("Warning headers do not match!");
        }
        l.set_error_header(header);
        if l.get_main_header(LogType::Err) !=
        l.colorify(header, l.log_header_color(LogType::Err)) {
            panic!("Error headers do not match!");
        }
        l.set_fatal_header(header);
        if l.get_main_header(LogType::FatalError) !=
        l.colorify(header, l.log_header_color(LogType::FatalError)) {
            panic!("Fatal error headers do not match!");
        }
    }

    #[test]
    fn test_log_colors() {
        // Test if colorify works
        let l = Logger::default();
        if l.colorify("a", Color::Red) != "\x1b[31ma\x1b[0m"
        {
            panic!("Failed to colorify a string!");
        }
    }

    #[test]
    fn test_templates() {
        let file_name = "/templates/test.json";
        match get_current_dir() {
            Ok(current_dir) => {
                let path = current_dir
                    .to_str()
                    .map(|s| s.to_string() + file_name)
                    .unwrap_or_else(|| String::from(file_name));

                let mut l = Logger::default();
                l.save_template(&path);
                l = Logger::from_template(&path);

                if l != Logger::default() {
                    panic!("Templates are not the same!");
                }
            }
            Err(e) => {
                eprintln!("Error getting current directory: {}", e);
            }
        }
    }

    #[test]
    fn test_formats() {
        let mut l = Logger::default();

        l.set_datetime_format("aaa");
        l.set_debug_header("d");
        l.set_info_header("i");
        l.set_warning_header("W");
        l.set_error_header("E");
        l.set_fatal_header("!");
        let _ = l.set_log_format("<l> <h>%h</h> <d>%d</d> <m>%m</m> </l>");

        let mut logstruct = LogStruct {
            datetime: l.get_datetime(),
            log_type: LogType::Debug,
            message: "aaa".to_string(),
        };
        let mut comp = format!("<l> <h>{}</h> <d>aaa</d> <m>aaa</m> </l>\n",
            l.colorify("d", l.log_header_color(LogType::Debug))
        );

        if l.format_log(&logstruct) != comp {
            panic!("Bad log formatting, expected \n'{}', got \n'{}'",
                comp,
                l.format_log(&logstruct));
        }

        logstruct.log_type = LogType::Info;
        comp = format!("<l> <h>{}</h> <d>aaa</d> <m>aaa</m> </l>\n",
            l.colorify("i", l.log_header_color(LogType::Info))
        );
        if l.format_log(&logstruct) != comp {
            panic!("Bad log formatting, expected \n'{}', got \n'{}'",
                comp,
                l.format_log(&logstruct));
        }

        logstruct.log_type = LogType::Warning;
        comp = format!("<l> <h>{}</h> <d>aaa</d> <m>aaa</m> </l>\n",
            l.colorify("W", l.log_header_color(LogType::Warning))
        );
        if l.format_log(&logstruct) != comp {
            panic!("Bad log formatting, expected \n'{}', got \n'{}'",
                comp,
                l.format_log(&logstruct));
        }

        logstruct.log_type = LogType::Err;
        comp = format!("<l> <h>{}</h> <d>aaa</d> <m>aaa</m> </l>\n",
            l.colorify("E", l.log_header_color(LogType::Err))
        );
        if l.format_log(&logstruct) != comp {
            panic!("Bad log formatting, expected \n'{}', got \n'{}'",
                comp,
                l.format_log(&logstruct));
        }

        logstruct.log_type = LogType::FatalError;
        comp = format!("<l> <h>{}</h> <d>aaa</d> <m>aaa</m> </l>\n",
            l.colorify("!", l.log_header_color(LogType::FatalError))
        );
        if l.format_log(&logstruct) != comp {
            panic!("Bad log formatting, expected \n'{}', got \n'{}'",
                comp,
                l.format_log(&logstruct));
        }
    }

    #[test]
    fn test_file_logging() {
        let file_name = "/output.log";
        let max_size: usize = 16;
        let mut l = Logger::default();
        l.set_max_log_buffer_size(max_size);

        let current_dir = get_current_dir();

        match current_dir {
            Ok(current_dir) => {
                let path = current_dir
                    .to_str()
                    .map(|s| s.to_string() + file_name)
                    .unwrap_or_else(|| String::from(file_name));

                let result = l.set_log_file_path(&path);

                match result {
                    Ok(()) => {
                        let _ = l.toggle_file_logging(true);
                        let mut i = 0;
                        loop {
                            l.fatal(&format!("i: {}", i));

                            if i >= max_size {
                                break;
                            }
                            i += 1;
                        }
                    },
                    Err(_) => { panic!("Failed to set the log file path to
                        '{}'!", path) },
                }
            },
            Err(_) => { panic!("Failed to get current directory!") },
        }
    }
}
