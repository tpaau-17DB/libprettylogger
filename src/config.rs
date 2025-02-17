//! Contains various types used to customize `Logger` behavior.

/// Holds various types used to customize `Logger` behavior.
///
/// # enums:
/// * `Verbosity` -> Represents the verbosity level of a `Logger`.
/// * `OnDropPolicy` -> Defines on drop policy for file logging.
/// * `LogType` -> The type (severity) of a log.
///
/// # structs:
/// * `LogStruct` -> Represents a single log entry. You can turn it into a log
/// using `Logger`'s format_log(...) method.

use serde::{Serialize, Deserialize};
use std::fmt;
use std::fmt::{Display, Formatter};
use chrono::{Local, DateTime};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Default,
    Serialize, Deserialize)]
/// Used to set the verbosity of a logger.
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
    /// The date and gime at which the log was created.
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
