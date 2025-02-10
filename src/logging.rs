use crate::{
    colors::*, fileio::append_to_file, filtering::*
};
use chrono::{Local, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub enum LogType {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
    FatalError = 4,
}

#[derive(Clone)]
pub struct LogStruct {
    pub message: String,
    pub log_type: LogType,
    pub datetime: DateTime<Local>,
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
#[derive(Serialize, Deserialize)]
pub struct Logger {
    pub(crate) verbosity: Verbosity,
    pub(crate) filtering_enabled: bool,
    pub(crate) auto_spacing: bool,

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

    pub(crate) message_left: String,
    pub(crate) message_right: String,
    
    pub(crate) log_left: String,
    pub(crate) log_right: String,

    pub(crate) show_datetime: bool,
    pub(crate) datetime_format: String,
    pub(crate) datetime_header_left: String,
    pub(crate) datetime_header_right: String,

    pub(crate) file_logging_enabled: bool,
    pub(crate) log_file_path: String,

    pub(crate) log_buffer_max_size: usize,
    #[serde(skip)]
    pub(crate) log_buffer: Vec<LogStruct>,
}

impl Drop for Logger {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

impl PartialEq<Logger> for Logger {
    fn eq(&self, other: &Logger) -> bool {
        self.verbosity == other.verbosity &&
        self.filtering_enabled == other.filtering_enabled &&
        self.auto_spacing == other.auto_spacing &&

        self.log_color_enabled == other.log_color_enabled &&

        self.debug_color == other.debug_color &&
        self.info_color == other.info_color &&
        self.warning_color == other.warning_color &&
        self.error_color == other.error_color &&
        self.fatal_color == other.fatal_color &&

        self.debug_header == other.debug_header &&
        self.info_header == other.info_header &&
        self.warning_header == other.warning_header &&
        self.error_header == other.error_header &&
        self.fatal_header == other.fatal_header &&

        self.message_left == other.message_left &&
        self.message_right == other.message_right &&

        self.log_left == other.log_left &&
        self.log_right == other.log_right &&

        self.show_datetime == other.show_datetime &&
        self.datetime_format == other.datetime_format &&
        self.datetime_header_left == other.datetime_header_left &&
        self.datetime_header_right == other.datetime_header_right &&

        self.file_logging_enabled == other.file_logging_enabled &&
        self.log_file_path == other.log_file_path
    }
}

impl Logger {
    // INTERNAL METHODS

    pub(crate) fn print_log(&mut self, log: &LogStruct) {
        let log_str = self.format_log(log);

        if self.file_logging_enabled {
            self.log_buffer.push(log.clone());

            if self.log_buffer.len() >= self.log_buffer_max_size {
                let _ = self.flush_file_log_buffer();
            }
        }

        print!("{}", log_str);
    }

    pub(crate) fn format_log(&self, log: &LogStruct) -> String {
        let message = self.message_left.clone()
            +&log.message + &self.message_right;

        if self.auto_spacing {
            if self.show_datetime {
                return format!("{} {} {} {} {}\n",
                    self.log_left.clone(),
                    self.get_header(&log.log_type),
                    self.get_datetime_header(&log.datetime),
                    message.to_string(),
                    self.log_right,
                );
            }
            else {
                return format!("{} {} {}{} {}\n",
                    self.log_left.clone(),
                    self.get_header(&log.log_type),
                    self.get_datetime_header(&log.datetime),
                    message.to_string(),
                    self.log_right,
                );
            }
        }
        return format!("{}{}{}{}{}\n",
            self.log_left.clone(),
            self.get_header(&log.log_type),
            self.get_datetime_header(&log.datetime),
            message.to_string(),
            self.log_right,
        );
    }

    pub(crate) fn filter_log(&self, log_type: LogType) -> bool {
        return !self.filtering_enabled
            || ((log_type as i32) < self.verbosity.clone() as i32)
    }

    pub(crate) fn get_datetime_header(&self,
    datetime: &DateTime<Local>) -> String {
        if self.show_datetime {
            let datetime_formatted = datetime.format(&self.datetime_format);
            let left = &self.datetime_header_left;
            let right = &self.datetime_header_right;
            // and here is the missing space
            return format!("{left}{datetime_formatted}{right}");
        }
        else {
            return String::from("");
        }
    }

    pub(crate) fn get_datetime(&self) -> DateTime<Local> {
        return Local::now();
    }

    pub(crate) fn get_header(&self, log_type: &LogType) -> String {
        match log_type {
            LogType::Debug => { 
                self.colorify(&self.debug_header, self.get_color(log_type))
            }
            LogType::Info => {
                self.colorify(&self.info_header, self.get_color(log_type))
            }
            LogType::Warning => {
                self.colorify(&self.warning_header, self.get_color(log_type))
            }
            LogType::Error => {
                self.colorify(&self.error_header, self.get_color(log_type))
            }
            LogType::FatalError => {
                self.colorify(&self.fatal_header, self.get_color(log_type))
            }
        }
    }

    pub(crate) fn get_color(&self, log_type: &LogType) -> Color {
        match log_type {
            LogType::Debug => { self.debug_color.clone() }
            LogType::Info => { self.info_color.clone() }
            LogType::Warning => { self.warning_color.clone() }
            LogType::Error => { self.error_color.clone() }
            LogType::FatalError => { self.fatal_color.clone() }
        }
    }

    pub(crate) fn colorify(&self, text: &str, color: Color) -> String {
        if self.log_color_enabled {
            return get_color_code(color) + text + &RESET;
        }
        else {
            return text.to_string();
        }
    }

    pub(crate) fn flush_file_log_buffer(&mut self) -> Result<(), ()> {
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
                Err(())
            },
        }
    }


    // CONSTRUCTORS

    /// Returns a `Logger` instance with default configuration applied.
    pub fn default() -> Self {
        Logger {
            verbosity: Verbosity::Standard,
            filtering_enabled: true,
            auto_spacing: true,

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

            message_left: String::from(""),
            message_right: String::from(""),

            log_left: String::from(""),
            log_right: String::from(""),

            show_datetime: false,
            datetime_format: String::from("%Y-%m-%d %H:%M:%S"),
            datetime_header_left: "[".to_string(),
            datetime_header_right: "]".to_string(),

            file_logging_enabled: false,
            log_file_path: "".to_string(),

            log_buffer_max_size: 128,
            log_buffer: Vec::new(),
        }
    }


    // PUBLIC METHODS

    /// Flushes log buffer (if file logging is enabled, it writes it to a file),
    /// and then clears the log buffer.
    pub fn flush(&mut self) -> Result<(), ()> {
        if self.file_logging_enabled {
            self.flush_file_log_buffer()?;
        }
        return Ok(());
    }

    /// Prints a **debug log**.
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

    /// Prints a **debug log**, bypasses filtering.
    pub fn debug_no_filtering(&mut self, message: &str) {
        let log = LogStruct {
            message: message.to_string(),
            log_type: LogType::Debug,
            datetime: self.get_datetime(),
        };
        self.print_log(&log);
    }

    /// Prints **info log**.
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

    /// Prints **info log**, bypasses filtering.
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
            log_type: LogType::Error,
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
        if l.get_header(&LogType::Debug) != 
        l.colorify(header, l.get_color(&LogType::Debug)) {
            panic!("Debug headers do not match!");
        }
        l.set_info_header(header);
        if l.get_header(&LogType::Info) != 
        l.colorify(header, l.get_color(&LogType::Info)) {
            panic!("Info headers do not match!");
        }
        l.set_warning_header(header);
        if l.get_header(&LogType::Warning) !=
        l.colorify(header, l.get_color(&LogType::Warning)) {
            panic!("Warning headers do not match!");
        }
        l.set_error_header(header);
        if l.get_header(&LogType::Error) != 
        l.colorify(header, l.get_color(&LogType::Error)) {
            panic!("Error headers do not match!");
        }
        l.set_fatal_header(header);
        if l.get_header(&LogType::FatalError) != 
        l.colorify(header, l.get_color(&LogType::FatalError)) {
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
    fn test_datetime_header() {
        // Test if datetime header format is parsed correctly
        
        let mut logger = Logger::default();

        if logger.set_datetime_header_format("[{}]") == Err(()) {
            panic!("Error while setting datetime header format!");
        }
        if logger.set_datetime_header_format("{}") == Err(()) {
            panic!("Error while setting datetime header format!");
        }
        if logger.set_datetime_header_format("[]") != Err(()) {
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

    #[test]
    fn test_formats() {
        let mut l = Logger::default();

        l.toggle_show_datetime(true);

        l.set_datetime_format("aaa");
        let _ = l.set_log_format("<l>{}</l>");
        l.set_debug_header("<h>d</h>");
        l.set_info_header("<h>i</h>");
        l.set_warning_header("<h>W</h>");
        l.set_error_header("<h>E</h>");
        l.set_fatal_header("<h>!</h>");
        let _ = l.set_datetime_header_format("<d>{}</d>");
        let _ = l.set_message_format("<m>{}</m>");

        let mut logstruct = LogStruct {
            datetime: l.get_datetime(),
            log_type: LogType::Debug,
            message: "aaa".to_string(),
        };
        let mut comp = format!("<l> {} <d>aaa</d> <m>aaa</m> </l>\n",
            l.colorify("<h>d</h>", l.get_color(&LogType::Debug))
        );

        if l.format_log(&logstruct) != comp {
            panic!("Bad log formatting, expected \n'{}', got \n'{}'",
                comp,
                l.format_log(&logstruct));
        }

        logstruct.log_type = LogType::Info;
        comp = format!("<l> {} <d>aaa</d> <m>aaa</m> </l>\n",
            l.colorify("<h>i</h>", l.get_color(&LogType::Info))
        );
        if l.format_log(&logstruct) != comp {
            panic!("Bad log formatting, expected \n'{}', got \n'{}'",
                comp,
                l.format_log(&logstruct));
        }

        logstruct.log_type = LogType::Warning;
        comp = format!("<l> {} <d>aaa</d> <m>aaa</m> </l>\n",
            l.colorify("<h>W</h>", l.get_color(&LogType::Warning))
        );
        if l.format_log(&logstruct) != comp {
            panic!("Bad log formatting, expected \n'{}', got \n'{}'",
                comp,
                l.format_log(&logstruct));
        }

        logstruct.log_type = LogType::Error;
        comp = format!("<l> {} <d>aaa</d> <m>aaa</m> </l>\n",
            l.colorify("<h>E</h>", l.get_color(&LogType::Error))
        );
        if l.format_log(&logstruct) != comp {
            panic!("Bad log formatting, expected \n'{}', got \n'{}'",
                comp,
                l.format_log(&logstruct));
        }

        logstruct.log_type = LogType::FatalError;
        comp = format!("<l> {} <d>aaa</d> <m>aaa</m> </l>\n",
            l.colorify("<h>!</h>", l.get_color(&LogType::FatalError))
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
        let max_size = 16;
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
