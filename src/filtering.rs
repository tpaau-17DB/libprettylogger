use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Default,
    Serialize, Deserialize)]
pub enum Verbosity {
    #[default]
    All = 0,
    Standard = 1,
    Quiet = 2,
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
