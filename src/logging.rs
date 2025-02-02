use crate::{
    filtering::*,
    colors::*,
};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

pub enum LogType {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
    FatalError = 4,
}

pub struct LogStruct<'a> {
    pub message: &'a str,
    pub log_type: LogType,
}

/// A logger struct used to print logs.
///
/// You can create a `Logger` struct with default configuration:
/// `Logger::default()`
///
/// You can also initialize `Logger` instance from a JSON template file, like so:
/// `Logger::from_template("/path/to/template.json")`
///
/// After creating a fresh `Logger`, you can use one of the logging methods
/// to print messages:
/// * `logger.debug("debug");`
/// * `logger.info("info);`
/// * `logger.warning("warning");`
/// * `logger.error("error");`
/// * `logger.fatal("fatal error");`
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Logger {
    pub(crate) verbosity: Verbosity,
    pub(crate) filtering_enabled: bool,

    pub(crate) log_color_enabled: bool,

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

    pub(crate) show_datetime: bool,
    pub(crate) datetime_header_left: String,
    pub(crate) datetime_header_right: String,
}

impl Logger {
    // INTERNAL METHODS

    fn print_log(&self, log: &LogStruct) {
        print!("{}", self.format_log(log));
    }

    fn filter_log(&self, log_type: LogType) -> bool {
        return !self.filtering_enabled
            || ((log_type as i32) < self.verbosity.clone() as i32)
    }

    fn get_datetime_header(&self) -> String {
        let time = Local::now();
        let right = self.datetime_header_left.clone();
        let left = self.datetime_header_right.clone();
        return format!("{left}{time}{right}");
    }

    fn get_header(&self, log_type: &LogType) -> String {
        match log_type {
            LogType::Debug => { self.debug_header.clone() }
            LogType::Info => { self.info_header.clone() }
            LogType::Warning => { self.warning_header.clone() }
            LogType::Error => { self.error_header.clone() }
            LogType::FatalError => { self.fatal_header.clone() }
        }
    }

    fn get_color(&self, log_type: &LogType) -> Color {
        match log_type {
            LogType::Debug => { self.debug_color.clone() }
            LogType::Info => { self.info_color.clone() }
            LogType::Warning => { self.warning_color.clone() }
            LogType::Error => { self.error_color.clone() }
            LogType::FatalError => { self.fatal_color.clone() }
        }
    }

    fn format_log(&self, log: &LogStruct) -> String {
        if !self.show_datetime {
            return format!("{} {}\n",
                self.colorify(&self.get_header(&log.log_type),
                    self.get_color(&log.log_type)),
                log.message);
        }
        else {
            return format!("{} {} {}\n",
                self.colorify(&self.get_header(&log.log_type),
                    self.get_color(&log.log_type)),
                self.get_datetime_header(),
                log.message);
        }
    }

    fn colorify(&self, text: &str, color: Color) -> String {
        if self.log_color_enabled {
            return get_color_code(color) + text + &get_color_code(Color::None);
        }
        else {
            return text.to_string();
        }
    }


    // CONSTRUCTORS

    /// Returns a `Logger` with default configuration applied.
    pub fn default() -> Self {
        Logger {
            verbosity: Verbosity::Standard,
            filtering_enabled: true,

            log_color_enabled: true,

            debug_color: Color::Blue,
            info_color: Color::Green,
            warning_color: Color::Yellow,
            error_color: Color::Red,
            fatal_color: Color::Magenta,

            debug_header: "[DBG]".to_string(),
            info_header: "[INF]".to_string(),
            warning_header: "[WAR]".to_string(),
            error_header: "[ERR]".to_string(),
            fatal_header: "[FATAL]".to_string(),

            show_datetime: false,
            datetime_header_left: "[".to_string(),
            datetime_header_right: "]".to_string(),
        }
    }


    // PUBLIC METHODS

    /// Prints a **debug log**.
    pub fn debug(self, message: &str) {
        if self.filter_log(LogType::Debug)
        {
            return;
        }
        let log = LogStruct {
            message,
            log_type: LogType::Debug,
        };
        self.print_log(&log);
    }

    /// Prints a **debug log**, bypasses filtering.
    pub fn debug_no_filtering(self, message: &str) {
        let log = LogStruct {
            message,
            log_type: LogType::Debug,
        };
        self.print_log(&log);
    }

    /// Prints an **informative log**.
    pub fn info(self, message: &str) {
        if self.filter_log(LogType::Info)
        {
            return;
        }
        let log = LogStruct {
            message,
            log_type: LogType::Info,
        };
        self.print_log(&log);
    }

    /// Prints an **informative log**, bypasses filtering.
    pub fn info_no_filtering(self, message: &str) {
        let log = LogStruct {
            message,
            log_type: LogType::Info,
        };
        self.print_log(&log);
    }

    /// Prints a **warning**.
    pub fn warning(self, message: &str) {
        if self.filter_log(LogType::Warning)
        {
            return;
        }
        let log = LogStruct {
            message,
            log_type: LogType::Warning,
        };
        self.print_log(&log);
    }

    /// Prints a **warning**, bypasses filtering.
    pub fn warning_no_filtering(self, message: &str) {
        let log = LogStruct {
            message,
            log_type: LogType::Warning,
        };
        self.print_log(&log);
    }

    /// Prints an **error** (errors cannot be suppressed).
    pub fn error(self, message: &str) {
        // error messages cant get suppressed
        let log = LogStruct {
            message,
            log_type: LogType::Error,
        };
        self.print_log(&log);
    }

    /// Prints a **fatal error** (errors cannot be suppressed).
    pub fn fatal(self, message: &str) {
        // error messages cant get suppressed
        let log = LogStruct {
            message,
            log_type: LogType::FatalError,
        };
        self.print_log(&log);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env,
        io,
        path::PathBuf,
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
        if l.get_header(&LogType::Debug) != header {
            panic!("Debug headers do not match!");
        }
        l.set_info_header(header);
        if l.get_header(&LogType::Info) != header {
            panic!("Debug headers do not match!");
        }
        l.set_warning_header(header);
        if l.get_header(&LogType::Warning) != header {
            panic!("Debug headers do not match!");
        }
        l.set_error_header(header);
        if l.get_header(&LogType::Error) != header {
            panic!("Debug headers do not match!");
        }
        l.set_fatal_header(header);
        if l.get_header(&LogType::FatalError) != header {
            panic!("Debug headers do not match!");
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
    fn test_datetime_header() {
        // Test if datetime header format is parsed correctly
        
        let mut logger = Logger::default();

        if logger.set_datetime_header_format("[{}]") == 1 {
            panic!("Error while setting datetime header format!");
        }
        if logger.set_datetime_header_format("{}") == 1 {
            panic!("Error while setting datetime header format!");
        }
        if logger.set_datetime_header_format("[]") == 0 {
            panic!("Format invalid, but no error thrown!");
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
}
