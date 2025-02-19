use crate::*;
use crate::{
    colors::*, fileio::*, config::*,
};
use std::io::{Error, ErrorKind};

impl Logger {
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
    /// * **true**  -> logs will get filtered based on verbosity
    /// * **false** -> log filtering will be disabled globally
    pub fn toggle_log_filtering<I: Into<bool>>(&mut self, enabled: I) {
        self.filtering_enabled = enabled.into();
    }

    /// Toggles colored log headers.
    /// * `true`  -> Log headers will have colors
    /// * `false` -> No colors :(
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
    }

    /// Sets the log format.
    ///
    /// There are three placeholders in a log format string (you can place
    /// multiple placeholders of the same type in a format string):
    /// * `%c` -> Ascending log count starting at 1.
    /// * `%d` -> The timestamp.
    /// * `%h` -> The header indicating the log type (e.g., debug, error, etc.)
    /// * `%m` -> The log message (this placeholder is mandatory, you will get
    /// an error if you don't include this in your log format).
    ///
    /// # Example
    /// ```
    /// # use prettylogger::Logger;
    /// # let mut l = Logger::default();
    /// l.set_log_format("<l> <h>%h</h> <m>%m</m> </l>");
    /// l.error("lorem ipsum");
    /// ```
    ///
    /// Returns an error when the `%m` placeholder is missing.
    pub fn set_log_format(&mut self, format: &str) -> Result<(), String> {
        if format.contains("%m") {
            self.log_format = String::from(format);
            self.show_datetime = format.contains("%d");
            Ok(())
        }
        else {
            Err(String::from("Expected a message placeholder!"))
        }
    }

    /// Sets log file path.
    ///
    /// Returns an error if the path is inaccessible.
    ///
    /// [File logging documentation](https://github.com/tpaau-17DB/libprettylogger?tab=readme-ov-file#file-logging)
    ///
    /// # Example
    /// ```
    /// # use prettylogger::Logger;
    /// # let mut logger = Logger::default();
    /// // Set the log file path first:
    /// logger.set_log_file_path("/tmp/libprettylogger-tests/set_log_file_path.log");
    /// // Then enable file logging:
    /// logger.toggle_file_logging(true);
    /// ```
    pub fn set_log_file_path(&mut self, path: &str) -> Result<(), Error> {
        let path: &str = &expand_env_vars(&expand_tilde(path));
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
    /// This is because this method checks if the log file is writable. If the
    /// log file path is not set, or the file is not writable, enabling file
    /// logging will result in an error.
    ///
    /// # Example
    /// ```
    /// # use prettylogger::Logger;
    /// # let mut logger = Logger::default();
    /// # logger.set_log_file_path("/tmp/libprettylogger-tests/toggle_file_logging.log");
    /// // Set the log file path first:
    /// logger.set_log_file_path("/tmp/libprettylogger-tests/toggle_file_logging.log");
    /// // Then enable file logging:
    /// logger.toggle_file_logging(true);
    /// ```
    pub fn toggle_file_logging<I: Into<bool>>(&mut self, enabled: I)
    -> Result<(), Error> {
        if !enabled.into() {
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
    /// # Example
    /// ```
    /// # use prettylogger::Logger;
    /// let mut logger = Logger::default();
    /// logger.set_log_file_path("/tmp/libprettylogger-tests/set_max_log_buffer_size.log");
    /// logger.toggle_file_logging(true);
    ///
    /// // This will make `Logger` to flush the log buffer every 16 logs:
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
    /// ```
    pub fn set_max_log_buffer_size<I: Into<u32>>(&mut self, size: I) {
        self.file_log_buffer_max_size = size.into();
    }

    /// Log file lock can be used to prevent race conditions when there is one
    /// thread reading from the log file and another thread writing to the log
    /// file.
    ///
    /// # WARNING: leaving this option on for a long period of time will cause
    /// high memory usage!
    ///
    /// * `true`  -> When log file lock is enabled, logger won't flush into the
    /// log file. Instead, it will wait until the lock is disabled. You will
    /// not loose any logs, they will be stored in the log buffer even when it
    /// exceeds its size limit.
    /// * `false` -> Logger will write to the log file normally.
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
    /// * `true` -> Logs will be printed in your terminal's `stdout`.
    /// * `false` -> No log output in your terminal.
    pub fn toggle_stdout<I: Into<bool>>(&mut self, enabled: I) {
        self.stdout_enabled = enabled.into();
    }

    /// Toggles the usage of a custom log buffer.
    /// * `true` -> Logs will be stored in a buffer inside `Logger` and can be
    /// cloned using the `clone_log_buffer()` method. Be aware that this will
    /// lead to high memory usage if turned on for a log period of time.
    /// * `false` -> Logs will not be stored in a log buffer.
    pub fn toggle_custom_log_buffer<I: Into<bool>>(&mut self, enabled: I) {
        self.use_custom_log_buffer = enabled.into();
    }

    /// Clears the custom log buffer.
    pub fn clear_log_buffer(&mut self) {
        self.custom_log_buffer = Vec::new();
    }
}
