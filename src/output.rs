//! Provides log stream implementations for directing log output to various
//! destinations, such as files, standard error, or a log buffer.

/// Provides log stream implementations for directing log output to various
/// destinations, such as files, standard error, or a log buffer.
use std::fs::OpenOptions;

use serde::{Serialize, Deserialize};

use crate::{
    Error,
    config::{LogStruct, OnDropPolicy},
    format::LogFormatter,
    fileio::{append_to_file, overwrite_file},
};

/// Common trait for toggleable objects.
pub trait Toggleable {
    /// Enables the object.
    fn enable(&mut self);
    /// Disables the object.
    fn disable(&mut self);
    /// Returns if the object is enabled.
    fn is_enabled(&self) -> &bool;
}

/// Wraps `StderrStream`, `BufferStream` and `FileStream` in one object used
/// internally by `Logger`.
///
/// # Examples
///
/// Printing log to `stderr`:
/// ```
/// # use prettylogger::{
/// #     output::LogOutput,
/// #     format::LogFormatter,
/// #     config::LogStruct,
/// # };
/// // Required by `LogOutput` for parsing logs
/// let mut formatter = LogFormatter::default();
///
/// // By default, only output to `stderr` is enabled
/// let mut log_output = LogOutput::default();
///
/// // Print "Hello, World!" in a neat log format
/// log_output.out(&LogStruct::debug("Hello, World!"), &mut formatter);
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize,
    Deserialize)]
pub struct LogOutput {
    /// The `stderr` output stream.
    pub stderr_output: StderrStream,
    /// File output stream for writing logs to a file.
    pub file_output: FileStream,
    /// Buffer stream for storing log messages.
    pub buffer_output: BufferStream,

    enabled: bool,
}

/// The `stderr` output stream.
///
/// # Examples
///
/// Printing a log to `stderr`:
/// ```
/// # use prettylogger::{
/// #     output::StderrStream,
/// #     format::LogFormatter,
/// #     config::LogStruct,
/// # };
/// // Required by `StderrStream` for parsing logs
/// let mut formatter = LogFormatter::default();
///
/// // Enabled by default
/// let mut stderr_output = StderrStream::default();
///
/// // Print "Hello, World!" in a neat log format
/// stderr_output.out(&LogStruct::debug("Hello, World!"), &mut formatter);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize,
    Deserialize)]
pub struct StderrStream {
    enabled: bool,
}

/// The file output stream.
///
/// # Examples
///
/// Writing a log to a file:
/// ```
/// # use prettylogger::{
/// #     output::{FileStream, Toggleable},
/// #     format::LogFormatter,
/// #     config::LogStruct,
/// # };
/// # let mut path = std::env::temp_dir();
/// # path.push("libprettylogger-tests/fo-struct-doc.log");
/// # let path = &path.to_str().unwrap().to_string();
/// // Required by `FileStream` for parsing logs
/// let mut formatter = LogFormatter::default();
///
/// let mut file_output = FileStream::default();
///
/// // Set the log file path **first**
/// file_output.set_log_file_path(&path)
///     .expect("Failed to set the log file path!");
///
/// // Enable the output
/// file_output.enable()
///     .expect("Failed to enable the output!");
///
/// // Write to the log file buffer
/// file_output.out(&LogStruct::debug("Hello from file!"), &mut formatter)
///     .expect("Failed to write to the buffer!");
///
/// // Flush the logs from the buffer to the log file
/// file_output.flush();
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize,
    Deserialize)]
pub struct FileStream {
    enabled: bool,
    max_buffer_size: Option<usize>,
    on_drop_policy: OnDropPolicy,

    #[serde(skip)]
    lock_enabled: bool,
    #[serde(skip)]
    log_file_path: String,
    #[serde(skip)]
    log_buffer: Vec<String>,
}

/// The buffer stream.
///
/// # Examples
/// ```
/// # use prettylogger::{
/// #     output::{BufferStream, Toggleable},
/// #     config::LogStruct,
/// # };
/// let mut buffer_output = BufferStream::default();
///
/// // `BufferStream` is disabled by default
/// buffer_output.enable();
///
/// // A formatter is not needed since `BufferStream` stores raw logs
/// buffer_output.out(&LogStruct::debug("Hello from buffer!"));
///
/// // Obtain a reference to the log buffer
/// let buffer = buffer_output.get_log_buffer();
/// ````
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize,
    Deserialize, Default)]
pub struct BufferStream {
    enabled: bool,

    #[serde(skip)]
    pub(crate) log_buffer: Vec<LogStruct>,
}

impl Drop for FileStream {
    fn drop(&mut self) {
        let _ = self.internal_flush(true);
    }
}

impl Default for LogOutput {
    fn default() -> Self {
        LogOutput {
            enabled: true,
            stderr_output: StderrStream::default(),
            file_output: FileStream::default(),
            buffer_output: BufferStream::default(),
        }
    }
}

impl Default for StderrStream {
    fn default() -> Self {
        StderrStream {
            enabled: true,
        }
    }
}

impl Default for FileStream {
    fn default() -> Self {
        FileStream {
            enabled: false,
            max_buffer_size: Some(128),
            on_drop_policy: OnDropPolicy::default(),

            lock_enabled: false,
            log_file_path: String::from(""),
            log_buffer: Vec::new(),
        }
    }
}

impl Toggleable for LogOutput {
    /// Enables the output.
    fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables the output.
    fn disable(&mut self) {
        self.enabled = false;
    }

    /// Returns if the output is enabled.
    fn is_enabled(&self) -> &bool {
        return &self.enabled;
    }
}

impl Toggleable for StderrStream {
    /// Enables the output.
    fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables the output.
    fn disable(&mut self) {
        self.enabled = false;
    }

    /// Returns if the output is enabled.
    fn is_enabled(&self) -> &bool {
        return &self.enabled;
    }
}

impl Toggleable for BufferStream {
    /// Enables the output.
    fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables the output.
    fn disable(&mut self) {
        self.enabled = false;
    }

    /// Returns if the output is enabled.
    fn is_enabled(&self) -> &bool {
        return &self.enabled;
    }
}

impl LogOutput {
    /// Passes the log and its formatter to child streams for processing.
    pub fn out(&mut self, log: &LogStruct, formatter: &mut LogFormatter) {
        if self.enabled {
            self.stderr_output.out(log, formatter);
            let _ = self.file_output.out(log, formatter);
            self.buffer_output.out(log);
        }
    }
}

impl StderrStream {
    /// Formats the given log using a formatter and prints it to `stderr`.
    pub fn out(self, log: &LogStruct, formatter: &mut LogFormatter) {
        if self.enabled {
            eprint!("{}", formatter.format_log(log));
        }
    }
}

impl FileStream {
    fn push_to_buffer(&mut self, log: String) -> Result<(), Error> {
        if !self.enabled {
            return Err(Error::new("Output disabled!"));
        }

        self.log_buffer.push(log);

        match self.max_buffer_size {
            Some(size) => {
                if self.log_buffer.len() >= size {
                    return self.internal_flush(false);
                }
                else {
                    return Ok(());
                }
            },
            None => Ok(()),
        }
    }

    /// Write contents of the log buffer to the log file and clear the buffer.
    fn append_to_log_file(&mut self) -> Result<(), Error> {
        let buf = self.log_buffer.join("");
        self.log_buffer = Vec::new();
        return append_to_file(&self.log_file_path, &buf);
    }

    /// Handle flushing logic internally.
    pub(crate) fn internal_flush(&mut self, is_drop_flush: bool) -> Result<(), Error> {
        if !self.enabled {
            return Err(Error::new("Output not enabled!"));
        }

        if self.log_buffer.is_empty() {
            return Err(Error::new("Log buffer is empty!"));
        }

        if is_drop_flush {
            if self.lock_enabled {
                if self.on_drop_policy == OnDropPolicy::IgnoreLogFileLock {
                    return self.append_to_log_file();
                }
                else {
                    return Err(Error::new(
                        &format!("Lock is enabled and on drop policy se to '{}'!",
                        self.on_drop_policy)));
                }
            }
            else {
                return self.append_to_log_file();
            }
        }

        if self.lock_enabled {
            return Err(Error::new("Lock is enabled."));
        }
        else {
            return self.append_to_log_file();
        }
    }

    pub(crate) fn drop_flush(&mut self) {
        let _ = self.internal_flush(true);
    }

    /// Sets the log file path.
    ///
    /// # Examples
    /// ```
    /// # use prettylogger::{
    /// #     output::{FileStream, Toggleable},
    /// #     format::LogFormatter,
    /// #     config::LogStruct,
    /// # };
    /// # let mut path = std::env::temp_dir();
    /// # path.push("libprettylogger-tests/fo-set_log_file_path-doc.log");
    /// # let path = &path.to_str().unwrap().to_string();
    /// # let formatter = LogFormatter::default();
    /// # let mut file_output = FileStream::default();
    ///
    /// // Set the log file path **first**
    /// file_output.set_log_file_path(&path)
    ///     .expect("Failed to set the log file path!");
    ///
    /// // And then enable the output
    /// file_output.enable()
    ///     .expect("Failed to enable the output!");
    /// ```
    pub fn set_log_file_path(&mut self, path: &str) -> Result<(), Error> {
        match OpenOptions::new().write(true).create(true).truncate(true).open(path) {
            Ok(_) => {
                self.log_file_path = path.to_string();
                match overwrite_file(path, "") {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        return Err(Error::new(&e.message));
                    }
                }
            },
            Err(e) => Err(Error::new(&format!("{}", e))),
        }
    }

    /// Formats the given log using a formatter and stores it in a buffer until
    /// it is flushed.
    ///
    /// # Examples
    /// ```
    /// # use prettylogger::{
    /// #     output::{FileStream, Toggleable},
    /// #     format::LogFormatter,
    /// #     config::LogStruct,
    /// # };
    /// # let mut path = std::env::temp_dir();
    /// # path.push("libprettylogger-tests/fo-out-doc.log");
    /// # let path = &path.to_str().unwrap().to_string();
    /// # let mut formatter = LogFormatter::default();
    /// # let mut file_output = FileStream::default();
    ///
    /// // Set the log file path **first**
    /// file_output.set_log_file_path(&path)
    ///     .expect("Failed to set the log file path!");
    ///
    /// // And then enable the output
    /// file_output.enable()
    ///     .expect("Failed to enable the output!");
    ///
    /// // Write to the buffer 100 times
    /// for i in 0..100 {
    ///     file_output.out(&LogStruct::debug(&format!("Log number {}", i)),
    ///         &mut formatter).expect("Failed to write to the buffer!");
    /// }
    ///
    /// // Write the log buffer contents to the log file
    /// file_output.flush();
    /// ```
    pub fn out(&mut self, log: &LogStruct, formatter: &mut LogFormatter)
        -> Result<(), Error> {
        return self.push_to_buffer(formatter.format_log(log));
    }

    /// Flush the contents of the log buffer to the log file.
    ///
    /// # Examples
    /// ```
    /// # use prettylogger::{
    /// #     output::{FileStream, Toggleable},
    /// #     format::LogFormatter,
    /// #     config::LogStruct,
    /// # };
    /// # let mut path = std::env::temp_dir();
    /// # path.push("libprettylogger-tests/fo-out-doc.log");
    /// # let path = &path.to_str().unwrap().to_string();
    /// # let mut formatter = LogFormatter::default();
    /// # let mut file_output = FileStream::default();
    ///
    /// // Set the log file path **first**
    /// file_output.set_log_file_path(&path)
    ///     .expect("Failed to set the log file path!");
    ///
    /// // And then enable the output
    /// file_output.enable()
    ///     .expect("Failed to enable the output!");
    ///
    /// file_output.out(&LogStruct::debug(&format!("Hello from file!")),
    ///     &mut formatter).expect("Failed to write to the buffer!");
    ///
    /// // Write the log buffer contents to the log file
    /// file_output.flush();
    /// ```
    pub fn flush(&mut self) -> Result<(), Error> {
        return self.internal_flush(false);
    }

    /// Sets the maximum size of the log buffer.
    ///
    /// When the buffer exceeds this size, its contents are written to a file
    /// and then cleared.
    ///
    /// # Examples
    /// ```
    /// # use prettylogger::{
    /// #     output::{FileStream, Toggleable},
    /// #     format::LogFormatter,
    /// #     config::LogStruct,
    /// # };
    /// # let mut path = std::env::temp_dir();
    /// # path.push("libprettylogger-tests/fo-set_max_buffer_size-doc.log");
    /// # let path = &path.to_str().unwrap().to_string();
    /// # let mut formatter = LogFormatter::default();
    /// # let mut file_output = FileStream::default();
    /// // Set the log file path **first**
    /// file_output.set_log_file_path(&path)
    ///     .expect("Failed to set the log file path!");
    ///
    /// // And then enable the output
    /// file_output.enable()
    ///     .expect("Failed to enable the output!");
    ///
    /// // Define the maximum buffer size
    /// let max_size = 128;
    /// file_output.set_max_buffer_size(Some(max_size));
    /// for i in 0..max_size {
    ///     // Write to the buffer
    ///     file_output.out(&LogStruct::debug(&format!("Log number {}", i)),
    ///         &mut formatter).expect("Failed to write to the buffer!");
    /// }
    /// // Here the buffer will be flushed to the log file.
    /// ```
    pub fn set_max_buffer_size<I: Into<Option<usize>>>(&mut self, size: I) {
        self.max_buffer_size = size.into();
    }

    /// Enables the output.
    ///
    /// Returns an error if the log file is not writable.
    ///
    /// # Examples
    /// ```
    /// # use prettylogger::{
    /// #     output::{FileStream, Toggleable},
    /// #     format::LogFormatter,
    /// #     config::LogStruct,
    /// # };
    /// # let mut path = std::env::temp_dir();
    /// # path.push("libprettylogger-tests/fo-enable-doc.log");
    /// # let path = &path.to_str().unwrap().to_string();
    /// # let formatter = LogFormatter::default();
    /// # let mut file_output = FileStream::default();
    ///
    /// // Set the log file path **first**
    /// file_output.set_log_file_path(&path)
    ///     .expect("Failed to set the log file path!");
    ///
    /// // And then enable the output
    /// file_output.enable()
    ///     .expect("Failed to enable the output!");
    /// ```
    pub fn enable(&mut self) -> Result<(), Error> {
        if self.enabled {
            return Ok(());
        }
        else {
            match OpenOptions::new().write(true).create(true).truncate(true)
            .open(&self.log_file_path) {
                Ok(_) => {
                    self.enabled = true;
                    return Ok(());
                },
                Err(e) => Err(Error::new(&format!("{}", e))),
            }
        }
    }

    /// Disables the output.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Sets the policy for handling the log buffer lock when the stream is
    /// dropped.
    pub fn set_on_drop_policy<I: Into<OnDropPolicy>>(&mut self, policy: I) {
        self.on_drop_policy = policy.into();
    }

    /// Locks the log file, preventing it from being written to.
    pub fn lock_file(&mut self) {
        self.lock_enabled = true;
    }

    /// Unlocks the log file, allowing the stream to write to it.
    pub fn unlock_file(&mut self) {
        self.lock_enabled = false;
    }

    /// Returns if the output is enabled.
    pub fn is_enabled(&self) -> &bool {
        return &self.enabled;
    }
}

impl BufferStream {
    /// Formats the given log using a formatter and stores it in a buffer.
    pub fn out(&mut self, log: &LogStruct) {
        if self.enabled {
            self.log_buffer.push(log.clone());
        }
    }

    /// Returns a reference to the internal log struct buffer.
    pub fn get_log_buffer(&self) -> &Vec<LogStruct> {
        return &self.log_buffer;
    }

    /// Clears the log buffer.
    pub fn clear(&mut self) {
        self.log_buffer = Vec::new();
    }
}
