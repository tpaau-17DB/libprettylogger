//! Contains various types used to customize `Logger` behavior.

/// Contains various types used to customize `Logger` behavior.
use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter};
use chrono::{Local, DateTime};

/// Used to set the verbosity of a logger.
///
/// # Example
/// ```
/// # use prettylogger::{Logger, config::Verbosity};
/// # let mut logger = Logger::default();
/// logger.set_verbosity(Verbosity::Quiet);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Default,
    Serialize, Deserialize)]
pub enum Verbosity {
    /// Display all logs
    All = 0,
    #[default]
    /// Just filter the debug logs
    Standard = 1,
    /// Only display errors and warnings
    Quiet = 2,
    /// Only display errors
    ErrorsOnly = 3,
}

/// Defines the policy for handling log file flushing when a `Logger` is
/// dropped.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default,
    Serialize, Deserialize)]
pub enum OnDropPolicy {
    /// Ignore log file lock and write to file anyway. This may cause race
    /// conditions
    IgnoreLogFileLock,
    #[default]
    /// Respect the log file lock and don't write to the log file. This may
    /// cause data loss
    DiscardLogBuffer,
}


/// Represents different types of log messages.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default,
    Serialize, Deserialize)]
pub enum LogType {
    /// A debug log
    Debug = 0,
    #[default]
    /// A standard, informative message
    Info = 1,
    /// A warning
    Warning = 2,
    /// An error
    Err = 3,
    /// A critical error
    FatalError = 4,
}

/// Represents a single log entry.
///
/// Can be used to create custom log messages or storing logs in memory for
/// later use.
///
/// # Example:
/// ```
/// # use prettylogger::{Logger, config::LogStruct};
/// # let mut logger = Logger::default();
/// // Get a formatted log message from a `LogStruct` instance:
/// let log_string = logger.format_log(&LogStruct::error("Much bad!"));
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct LogStruct {
    /// The log message
    pub message: String,
    /// The type of the log (e.g., `Debug`, `Info`, `Warning`, etc.)
    pub log_type: LogType,
    /// The date and time at which the log struct was instantiated
    pub datetime: DateTime<Local>,
}

impl LogStruct {
    /// Returns a `LogStruct` with **debug** preset applied.
    ///
    /// # Example
    /// ```
    /// # use prettylogger::config::LogStruct;
    /// let debug_log = LogStruct::debug("This is a debug log!");
    /// ```
    pub fn debug(message: &str) -> LogStruct {
        LogStruct {
            message: message.to_string(),
            log_type: LogType::Debug,
            datetime: Local::now(),
        }
    }

    /// Returns a `LogStruct` with **info** preset applied.
    ///
    /// # Example
    /// ```
    /// # use prettylogger::config::LogStruct;
    /// let debug_log = LogStruct::info("This is an info log!");
    /// ```
    pub fn info(message: &str) -> LogStruct {
        LogStruct {
            message: message.to_string(),
            log_type: LogType::Info,
            datetime: Local::now(),
        }
    }

    /// Returns a `LogStruct` with **warning** preset applied.
    ///
    /// # Example
    /// ```
    /// # use prettylogger::config::LogStruct;
    /// let debug_log = LogStruct::warning("This is a warning!");
    /// ```
    pub fn warning(message: &str) -> LogStruct {
        LogStruct {
            message: message.to_string(),
            log_type: LogType::Warning,
            datetime: Local::now(),
        }
    }

    /// Returns a `LogStruct` with **error** preset applied.
    ///
    /// # Example
    /// ```
    /// # use prettylogger::config::LogStruct;
    /// let debug_log = LogStruct::error("This is an error!");
    /// ```
    pub fn error(message: &str) -> LogStruct {
        LogStruct {
            message: message.to_string(),
            log_type: LogType::Err,
            datetime: Local::now(),
        }
    }

    /// Returns a `LogStruct` with **fatal error** preset applied.
    ///
    /// # Example
    /// ```
    /// # use prettylogger::config::LogStruct;
    /// let debug_log = LogStruct::fatal_error("This is a fatal error!");
    /// ```
    pub fn fatal_error(message: &str) -> LogStruct {
        LogStruct {
            message: message.to_string(),
            log_type: LogType::FatalError,
            datetime: Local::now(),
        }
    }
}

impl Display for LogStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Log: {}\nType: {:?}\nDateTime: {}",
            self.message,
            self.log_type,
            self.datetime
        )
    }
}


impl std::fmt::Display for Verbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let level_str = match *self {
            Verbosity::All => "All",
            Verbosity::Standard => "Standard",
            Verbosity::Quiet => "Quiet",
            Verbosity::ErrorsOnly => "ErrorsOnly",
        };
        return write!(f, "{}", level_str)
    }
}

impl TryFrom<i32> for Verbosity {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Verbosity::All),
            1 => Ok(Verbosity::Standard),
            2 => Ok(Verbosity::Quiet),
            3 => Ok(Verbosity::ErrorsOnly),
            _ => Err(String::from("Invalid value, please provide a value in range from 0 to 3.")),
        }
    }
}

impl AsRef<str> for Verbosity {
    fn as_ref(&self) -> &str {
        match self {
            Verbosity::All => "All",
            Verbosity::Standard => "Standard",
            Verbosity::Quiet => "Quiet",
            Verbosity::ErrorsOnly => "ErrorsOnly",
        }
    }
}


impl Display for OnDropPolicy {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let level_str = match *self {
            OnDropPolicy::IgnoreLogFileLock => "IgnoreLogFileLock",
            OnDropPolicy::DiscardLogBuffer => "DiscardLogBuffer",
        };
        return write!(f, "{}", level_str)
    }
}


impl TryFrom<i32> for LogType {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LogType::Debug),
            1 => Ok(LogType::Info),
            2 => Ok(LogType::Warning),
            3 => Ok(LogType::Err),
            4 => Ok(LogType::FatalError),
            _ => Err(String::from("Invalid value, please provide a value in range from 0 to 4.")),
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
        return write!(f, "{}", level_str)
    }
}

impl AsRef<str> for LogType {
    fn as_ref(&self) -> &str {
        match self {
            LogType::Debug => "Debug",
            LogType::Info => "Info",
            LogType::Warning => "Warning",
            LogType::Err => "Err",
            LogType::FatalError => "FatalError",
        }
    }
}
