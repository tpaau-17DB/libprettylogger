/// This module holds the implementation of `Verbosity` used for log filtering.

use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Default,
    Serialize, Deserialize)]
/// Defines logger verbosity levels.
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
