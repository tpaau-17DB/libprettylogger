//! Contains various types used to customize `Logger` behavior.

/// Contains various types used to customize `Logger` behavior.
/// If you are good with the default `Logger` preset, you probably don't need
/// to use this module.

use serde::{Serialize, Deserialize};
use std::fmt;
use std::fmt::{Display, Formatter};
use chrono::{Local, DateTime};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Default,
    Serialize, Deserialize)]
/// Used to set the verbosity of a logger.
///
/// # Example
/// ```
/// # use prettylogger::{Logger, config::Verbosity};
/// # let mut logger = Logger::default();
/// logger.set_verbosity(Verbosity::Quiet);
/// ```
pub enum Verbosity {
    /// Don't filter any logs.
    All = 0,
    #[default]
    /// Just filter debug logs.
    Standard = 1,
    /// Only let warnings and errors to be displayed.
    Quiet = 2,
    /// I'm not gonna explain this one
    ErrorsOnly = 3,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default,
    Serialize, Deserialize)]
/// Defines the policy for handling log file flushing when a `Logger` is
/// dropped.
///
/// The default policy is `DiscardLogBuffer`.
pub enum OnDropPolicy {
    /// Completely ignore log file lock and write to file anyway.
    IgnoreLogFileLock,
    #[default]
    /// Don't write to the log file when lock is enabled.
    DiscardLogBuffer,
}


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default,
    Serialize, Deserialize)]
/// Represents different types of log messages.
///
/// This enum is used to categorize the severity or type of a log message.
/// The variants correspond to different levels of logging, from debugging
/// information to fatal errors.
///
/// The default variant is `Info`.
pub enum LogType {
    /// A debug log, used for detailed information during development.
    Debug = 0,
    #[default]
    Info = 1,
    Warning = 2,
    Err = 3,
    /// A critical error that leads to an unrecoverable state.
    FatalError = 4,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
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
pub struct LogStruct {
    /// The log message.
    pub message: String,
    /// The type of the log (e.g., `Debug`, `Error`, `Info`, etc.).
    pub log_type: LogType,
    /// The date and time at which the log was created.
    pub datetime: DateTime<Local>,
}

impl LogStruct {
    /// Returns a `LogStruct` with **debug** preset applied.
    pub fn debug(message: &str) -> LogStruct {
        LogStruct {
            message: message.to_string(),
            log_type: LogType::Debug,
            datetime: Local::now(),
        }
    }

    /// Returns a `LogStruct` with **info** preset applied.
    pub fn info(message: &str) -> LogStruct {
        LogStruct {
            message: message.to_string(),
            log_type: LogType::Info,
            datetime: Local::now(),
        }
    }

    /// Returns a `LogStruct` with **warning** preset applied.
    pub fn warning(message: &str) -> LogStruct {
        LogStruct {
            message: message.to_string(),
            log_type: LogType::Warning,
            datetime: Local::now(),
        }
    }

    /// Returns a `LogStruct` with **error** preset applied.
    pub fn error(message: &str) -> LogStruct {
        LogStruct {
            message: message.to_string(),
            log_type: LogType::Err,
            datetime: Local::now(),
        }
    }

    /// Returns a `LogStruct` with **fatal error** preset applied.
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
        write!(
            f,
            "Log: {}\nType: {:?}\nDateTime: {}",
            self.message,
            self.log_type,
            self.datetime
        )
    }
}


impl fmt::Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let level_str = match *self {
            Verbosity::All => "All",
            Verbosity::Standard => "Standard",
            Verbosity::Quiet => "Quiet",
            Verbosity::ErrorsOnly => "ErrorsOnly",
        };
        write!(f, "{}", level_str)
    }
}

impl TryFrom<i32> for Verbosity {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Verbosity::All),
            1 => Ok(Verbosity::Standard),
            2 => Ok(Verbosity::Quiet),
            3 => Ok(Verbosity::ErrorsOnly),
            _ => Err("Invalid value! Please provide a value in range 0-9."),
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
        write!(f, "{}", level_str)
    }
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
