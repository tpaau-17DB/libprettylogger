//! The `libprettylogger` crate offers a flexible logging utility through the
//! `Logger` struct. It handles automatic log filtering and formatting, allowing
//! you to easily control log output. The crate supports customizable log formats
//! and provides different log levels (debug, info, warning, error, fatal),
//! enabling precise control over logging behavior. Additionally, `Logger`
//! configuration can be saved as a JSON file, allowing you to easily manage
//! logging settings across different environments and use cases.

mod fileio;
mod setters;
mod json;

pub mod colors;
pub mod config;

use fileio::append_to_file;
use chrono::{Local, DateTime};
use serde::{Serialize, Deserialize};
use colors::*;
use config::*;

/// The `Logger` struct used to print logs.
///
/// # You can create a `Logger` instance with the default configuration using:
/// ```
/// # use prettylogger::Logger;
/// let logger = Logger::default();
/// ```
///
/// # Alternatively, initialize a `Logger` instance from a JSON template file:
/// ```ignore
/// Logger::from_template("/path/to/file.json");
/// ```
///
/// # Once you have a `Logger` instance, you can start printing logs:
/// ```
/// # use prettylogger::Logger;
/// # let mut logger = Logger::default();
/// logger.debug("debug message");
/// logger.info("info message");
/// logger.warning("warning message");
/// logger.error("error message");
/// logger.fatal("fatal error message");
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default,
    Serialize, Deserialize)]
pub struct Logger {
    pub(crate) stdout_enabled: bool,

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
    pub(crate) file_log_buffer_max_size: usize,
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

impl Drop for Logger {
    fn drop(&mut self) {
        let _ = self.drop_flush();
    }
}

impl Logger {
    // INTERNAL METHODS

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
        if self.filtering_enabled {
            return (log_type as i32) < self.verbosity as i32;
        }
        return false;
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

        for log in &self.file_log_buffer {
            buf += &self.format_log(&log);
        }

        self.file_log_buffer = Vec::new();
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
            stdout_enabled: true,

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

            log_format: String::from("[%h] %m"),
            datetime_format: String::from("%Y-%m-%d %H:%M:%S"),

            file_logging_enabled: false,
            log_file_path: String::new(),
            file_log_buffer_max_size: 128,
            on_drop_policy: OnDropPolicy::default(),

            custom_log_buffer: Vec::new(),
            file_log_buffer: Vec::new(),
            show_datetime: false,
            log_file_lock: false,
            log_count: 1,
        }
    }


    // PUBLIC METHODS

    /// Used to print a log out of a `LogStruct` structure.
    pub fn print_log(&mut self, log: &LogStruct) {
        self.log_count += 1;
        let log_str = self.format_log(log);

        if self.stdout_enabled {
            print!("{}", log_str);
        }

        if self.use_custom_log_buffer {
            self.custom_log_buffer.push(log.clone());
        }

        if self.file_logging_enabled {
            self.file_log_buffer.push(log.clone());

            if self.file_log_buffer_max_size != 0
            && self.file_log_buffer.len() >= self.file_log_buffer_max_size {
                let _ = self.flush_file_log_buffer(false);
            }
        }
    }

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
                            }
                            'd' => {
                                result += &headers.1;
                            }
                            'm' => {
                                result += &headers.2;
                            }
                            'c' => {
                                result += &self.log_count.to_string();
                            }
                            _ => {
                                result += &nc.to_string();
                            }
                        }
                        char_iter.next();
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
        let log = LogStruct::debug(message);
        self.print_log(&log);
    }

    /// Prints a **debug message**, bypasses filtering.
    pub fn debug_no_filtering(&mut self, message: &str) {
        let log = LogStruct::debug(message);
        self.print_log(&log);
    }

    /// Prints **info message**.
    pub fn info(&mut self, message: &str) {
        if self.filter_log(LogType::Info)
        {
            return;
        }
        let log = LogStruct::info(message);
        self.print_log(&log);
    }

    /// Prints **info message**, bypasses filtering.
    pub fn info_no_filtering(&mut self, message: &str) {
        let log = LogStruct::info(message);
        self.print_log(&log);
    }

    /// Prints a **warning**.
    pub fn warning(&mut self, message: &str) {
        if self.filter_log(LogType::Warning)
        {
            return;
        }
        let log = LogStruct::warning(message);
        self.print_log(&log);
    }

    /// Prints a **warning**, bypasses filtering.
    pub fn warning_no_filtering(&mut self, message: &str) {
        let log = LogStruct::warning(message);
        self.print_log(&log);
    }

    /// Prints an **error** (errors cannot be suppressed).
    pub fn error(&mut self, message: &str) {
        let log = LogStruct::error(message);
        self.print_log(&log);
    }

    /// Prints a **fatal error** (errors cannot be suppressed).
    pub fn fatal(&mut self, message: &str) {
        let log = LogStruct::fatal_error(message);
        self.print_log(&log);
    }

    /// Returns a clone of the custom log buffer.
    pub fn clone_log_buffer(&self) -> Vec<LogStruct> {
        return self.custom_log_buffer.clone();
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
        l.toggle_log_filtering(false);
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

        let mut logstruct = LogStruct::debug("aaa");
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

    #[test]
    fn test_custom_log_buffer() {
        let iter = 100;
        let mut logger = Logger::default();
        logger.toggle_log_filtering(false);
        logger.toggle_custom_log_buffer(true);

        let mut i = 0;
        loop {
            logger.debug("debug");
            i += 1;
            if i > iter - 1 {
                break;
            }
        }

        let log_buffer = logger.clone_log_buffer();

        for log in &log_buffer {
            if log.message != "debug" {
                panic!("Unexpected log message!");
            }
            if log.log_type != LogType::Debug {
                panic!("Unexpected log type!");
            }
        }

        if log_buffer.len() != iter {
            panic!("Expected a buffer size of {}, got {}.",
                iter,
                log_buffer.len());
        }
    }
}
