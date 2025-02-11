use crate::{
    colors::*, fileio::*, filtering::*, logging::*
};
use std::io::{Error, ErrorKind};

fn format_brackets(format: &str) -> Result<(String, String), &str> {
    let mut open_index = None;
    let mut close_index = None;

    for (i, c) in format.char_indices() {
        if c == '{' {
            open_index = Some(i);
        }
        else if c == '}' && open_index.unwrap() == i - 1 {
            close_index = Some(i);
            break;
        }
        else {
            open_index = None;
            close_index = None;
        }
    }

    if open_index.is_none() || close_index.is_none() 
    {
        return Err("Failed to set the format! Malformed or nonexistent placeholder.");
    }
    else {
        let opening = &format[0..open_index.unwrap()];
        let closing = &format[close_index.unwrap() + 1..format.len()];
        return Ok((String::from(opening), String::from(closing)));
    }
}

impl Logger {
    /// Sets logger `verbosity`.
    /// * `All` -> Don't filter any logs
    /// * `Standard` -> Just filter debug logs
    /// * `Quiet` -> Only let warnings and errors to be displayed
    /// * `ErrorsOnly` -> I'm not gonna explain this one
    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.verbosity = verbosity;
    }

    /// Toggles log filtering.
    /// * **true**  -> logs will get filtered based on verbosity
    /// * **false** -> log filtering will be disabled globally
    pub fn toggle_log_filtering(&mut self, enabled: bool) {
        self.filtering_enabled = enabled;
    }

    /// Toggles colored log headers.
    /// * `true`  -> Log headers will have colors 
    /// * `false` -> No colors :(
    pub fn toggle_log_color(&mut self, enabled: bool) {
        self.log_color_enabled = enabled;
    }

    pub fn toggle_auto_spacing(&mut self, enabled: bool) {
        self.auto_spacing = enabled;
    }

    /// Sets **debug log header** color.
    pub fn set_debug_color(&mut self, color: Color) {
        self.debug_color = color;
    }

    /// Sets **info log header** color.
    pub fn set_info_color(&mut self, color: Color) {
        self.info_color = color;
    }

    /// Sets **warning header** color.
    pub fn set_warning_color(&mut self, color: Color) {
        self. warning_color = color;
    }

    /// Sets **error header** color.
    pub fn set_error_color(&mut self, color: Color) {
        self.error_color = color;
    }

    /// Sets **fatal error header** color.
    pub fn set_fatal_color(&mut self, color: Color) {
        self.fatal_color = color;
    }

    /// Sets **debug log header** format.
    pub fn set_debug_header(&mut self, format: &str) {
        self.debug_header = format.to_string();
    }

    /// Sets **info log header** format.
    pub fn set_info_header(&mut self, format: &str) {
        self.info_header = format.to_string();
    }

    /// Sets **warning header** format.
    pub fn set_warning_header(&mut self, format: &str) {
        self.warning_header = format.to_string();
    }

    /// Sets **error header** format.
    pub fn set_error_header(&mut self, format: &str) {
        self.error_header = format.to_string();
    }

    /// Sets **fatal error header** format.
    pub fn set_fatal_header(&mut self, format: &str) {
        self.fatal_header = format.to_string();
    }

    /// Sets datetime header format.
    ///
    /// Note that this function sets the **header format** alone, 
    /// not the **datetime format**!
    ///
    /// Format must contain `{}`, so logger knows where to put the datetime
    /// string.
    ///
    /// For example, if you set the format to `<d>{}</d>`, 
    /// a datetime header will be displayed like this:
    /// **<d>2024-5-12 14:56</d>**
    pub fn set_datetime_header_format<'a>(&'a mut self, format: &'a str)
    -> Result<(), &'a str> {
        match format_brackets(format) {
            Ok(value) => {
                self.datetime_header_left = value.0;
                self.datetime_header_right = value.1;
                return Ok(());
            }
            Err(value) => { Err(value) }
        }
    }

    /// Sets datetime format.
    ///
    /// Note that this function sets the **datetime format** alone, not the
    /// **datetime header format**!
    pub fn set_datetime_format(&mut self, format: &str) {
        self.datetime_format = String::from(format);
    }

    /// Toggles datetime header for every log.
    /// * `true`  -> Every log will have a datetime header
    /// * `false` -> No datetime headers
    pub fn toggle_show_datetime(&mut self, enabled: bool) {
        self.show_datetime = enabled;
    }

    /// Sets message format.
    ///
    /// Format must contain the `{}` string, so logger knows where
    /// to put the message.
    ///
    /// For example, if you set the format to `<m>{}</m>`, 
    /// a message will be displayed like this:
    /// **<m>message!!!!!</m>**
    pub fn set_message_format<'a>(&'a mut self, format: &'a str)
    -> Result<(), &'a str> {
        match format_brackets(format) {
            Ok(value) => {
                self.message_left = value.0;
                self.message_right = value.1;
                return Ok(());
            }
            Err(value) => { Err(value) }
        }
    }

    /// Sets log format.
    ///
    /// Format must contain the `{}` string, so logger knows where
    /// to put the log.
    ///
    /// For example, if you set the format to `<l>{}</l>`,
    /// a log will be displayed like this:
    /// **<l>[LOG HEADER] [YYYY:MM:DD] Log message</l>**
    pub fn set_log_format<'a>(&'a mut self, format: &'a str)
    -> Result<(), &'a str> {
        match format_brackets(format) {
            Ok(value) => {
                self.log_left = value.0;
                self.log_right = value.1;
                return Ok(());
            }
            Err(value) => { Err(value) }
        }
    }

    /// Toggles file logging.
    ///
    /// Before enabling file logging, ensure that the log file path is set.
    /// This method will check if the specified log file is writable before 
    /// enabling file logging, to prevent errors related to file access.
    ///
    /// ```ignore
    /// # use prettylogger::logging::Logger;
    /// # let mut l = Logger::default();
    /// // We need to set log file path first:
    /// l.set_log_file_path("/path/to/file.log"); // a valid file path
    /// l.toggle_file_logging(true); // Then we can enable file logging
    /// ```
    pub fn toggle_file_logging(&mut self, enabled: bool) -> Result<(), Error> {
        if !enabled {
            self.file_logging_enabled = false;
            Ok(())
        }
        else {
            if is_file_writable(&self.log_file_path) {
                self.file_logging_enabled = true;
                Ok(())
            }
            else {
                self.error(&format!("Failed to open file '{}' for writing!",
                    self.log_file_path));
                Err(Error::new(ErrorKind::PermissionDenied,
                    "File is not writable!"))
            }
        }
    }

    /// Sets log file path.
    pub fn set_log_file_path(&mut self, path: &str) -> Result<(), Error> {
        if is_file_writable(path) {
            self.log_file_path = path.to_string();
            overwrite_file(path, "")
            .map_err(|e| {
                    self.error(&format!("Failed to open file '{}' for writing!",
                        self.log_file_path));
                    Error::new(ErrorKind::Other,
                        format!("Failed to overwrite file: {}", e))
                })?;
            Ok(())
        } 
        else {
            self.error(&format!("Failed to open file '{}' for writing!",
                self.log_file_path));
            Err(Error::new(ErrorKind::PermissionDenied,
                "File is not writable!"))
        }
    }

    /// Sets the maximum size of a log buffer.
    ///
    /// When log buffer exceeds this value, it will be flushed automatically.
    ///
    /// When set to `0`, log buffer won't be flushed automatically.
    pub fn set_max_log_buffer_size(&mut self, size: usize) {
        self.log_buffer_max_size = size;
    }
}

mod tests {
    // linter bug?
    #[allow(unused_imports)]
    use super::*; 

    #[test]
    fn test_bracket_formatting() {
        match format_brackets("{]]") {
            Err(_) => {},
            _ => panic!("format_brackets should throw an error!"),
        }

        match format_brackets("aaaaaaaa") {
            Err(_) => {},
            _ => panic!("format_brackets should throw an error!"),
        }

        match format_brackets("{}") {
            Ok(_) => {},
            Err(_) => panic!("format_brackets shouldn't throw an error!"),
        }
    }
}
