use crate::{
    colors::*, fileio::*, filtering::*, logging::*
};
use std::io::{Error, ErrorKind};

impl Logger {
    /// Sets logger `verbosity`.
    /// * `All` -> Don't filter any logs
    /// * `Standard` -> Just filter debug logs
    /// * `Quiet` -> Only let warnings and errors to be displayed
    /// * `ErrorsOnly` -> I'm not gonna explain this one
    pub fn set_verbosity(&mut self, verbosity: &Verbosity) {
        self.verbosity = *verbosity;
    }

    /// Toggles log filtering.
    /// * **true**  -> logs will get filtered based on verbosity
    /// * **false** -> log filtering will be disabled globally
    pub fn toggle_log_filtering(&mut self, enabled: &bool) {
        self.filtering_enabled = *enabled;
    }

    /// Toggles colored log headers.
    /// * `true`  -> Log headers will have colors 
    /// * `false` -> No colors :(
    pub fn toggle_log_header_color(&mut self, enabled: &bool) {
        self.log_header_color_enabled = *enabled;
    }

    /// Sets **debug log header** color.
    pub fn set_debug_color(&mut self, color: &Color) {
        self.debug_color = *color;
    }

    /// Sets **info log header** color.
    pub fn set_info_color(&mut self, color: &Color) {
        self.info_color = *color;
    }

    /// Sets **warning header** color.
    pub fn set_warning_color(&mut self, color: &Color) {
        self. warning_color = *color;
    }

    /// Sets **error header** color.
    pub fn set_error_color(&mut self, color: &Color) {
        self.error_color = *color;
    }

    /// Sets **fatal error header** color.
    pub fn set_fatal_color(&mut self, color: &Color) {
        self.fatal_color = *color;
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
    }

    /// Sets the log format.
    ///
    /// There are three placeholders in a log format string (you can place
    /// multiple placeholders of the same type in a format string):
    /// * `%m` -> The log message (this placeholder is mandatory, you format
    /// will get rejected if it doesn't contain this placeholder)
    /// * `%h` -> The log type header (debug, error, etc.)
    /// * `%d` -> The datetime
    ///
    /// # Set an XML-like log format;
    /// ```
    /// # use prettylogger::logging::Logger;
    /// # let mut l = Logger::default();
    /// l.set_log_format("<l> <h>%h</h> <m>%m</m> </l>");
    /// l.error("A nice debug log!");
    /// ```
    ///
    /// Returns an error when the `%m` placeholder is missing.
    pub fn set_log_format(&mut self, format: &str) -> Result<(), &'static str> {
        if format.contains("%m") {
            self.log_format = String::from(format);
            self.show_datetime = format.contains("%d");
            Ok(())
        }
        else {
            Err("Expected a message placeholder ('%m')!")
        }
    }

    /// Sets log file path.
    ///
    /// Returns an error if the path is inaccessible.
    ///
    /// # Make sure to actually enable file logging:
    /// ```ignore
    /// # use prettylogger::logging::Logger;
    /// # let mut logger = Logger::default();
    /// // Set the log file path first:
    /// logger.set_log_file_path("/path/to/file.log");
    /// // Then enable file logging:
    /// logger.toggle_file_logging(true);
    /// ```
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

    /// Toggles file logging.
    ///
    /// Before enabling file logging, ensure that the log file path is set. 
    /// This is because the method checks if the log file is writable. If the
    /// log  file path is not set, or the file is not writable, enabling file
    /// logging will result in an error.
    ///
    /// # Enabling file logging:
    /// ```ignore
    /// # use prettylogger::logging::Logger;
    /// # let mut logger = Logger::default();
    /// // Set the log file path first:
    /// logger.set_log_file_path("/path/to/file.log");
    /// // Then enable file logging:
    /// logger.toggle_file_logging(true);
    /// ```
    pub fn toggle_file_logging(&mut self, enabled: &bool) -> Result<(), Error> {
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

    /// Sets the maximum size of the log buffer.
    ///
    /// This method sets the maximum allowed size for the log buffer. When the
    /// buffer exceeds this size, it will be automatically flushed to the log
    /// file. If the buffer size is set to `0`, automatic flushing is disabled,
    /// and the buffer can only be flushed manually.
    ///
    /// If a log file lock is active, the log buffer will not be flushed
    /// automatically, regardless of the size limit.
    ///
    /// # Setting a custom log buffer size:
    /// ```ignore
    /// let mut logger = Logger::default();
    /// logger.set_log_file_path("/path/to/file.log");
    /// logger.toggle_file_logging(true);
    ///
    /// // This will force `Logger` to flush the log buffer every 16 logs:
    /// logger.set_max_log_buffer_size(&16);
    ///
    /// let mut i = 0;
    /// loop {
    ///     logger.info("Yay!");
    ///     i += 1;
    ///     if i >= 16 {
    ///         break;
    ///     }
    /// }
    /// ```
    pub fn set_max_log_buffer_size(&mut self, size: &usize) {
        self.log_buffer_max_size = *size;
    }

    /// Log file lock can be used to prevent race conditions when there is one
    /// thread reading from the log file and another thread writing to the log
    /// file.
    ///
    /// # WARNING: LEAVING THIS OPTION **ON** FOR A LONG PERIOD OF TIME CAN CAUSE
    /// HIGH MEMORY USAGE AND STUTTERING!
    ///
    /// `true`  -> When log file lock is enabled, logger won't flush into the
    /// log file. Instead, it will wait until the lock is disabled. You will
    /// not loose any logs, they will be stored in the log buffer even when it
    /// exceeds its size limit.
    /// `false` -> Logger will write to the log file normally.
    pub fn toggle_log_file_lock(&mut self, enabled: &bool) {
        self.log_file_lock = *enabled;
    }

    /// Sets `Logger`'s on drop log file policy.
    ///
    /// ```
    /// # use prettylogger::logging::{Logger, OnDropPolicy};
    /// # let mut logger = Logger::default();
    /// logger.set_on_drop_file_policy(&OnDropPolicy::IgnoreLogFileLock);
    /// ```
    pub fn set_on_drop_file_policy(&mut self, policy: &OnDropPolicy) {
        self.on_drop_policy = *policy;
    }
}
