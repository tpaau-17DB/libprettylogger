use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Verbosity {
    All = 0,
    Standard = 1,
    Quiet = 2,
    ErrorsOnly = 3,
}

impl fmt::Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let level_str = match *self {
            Verbosity::All => "Verbosity::All",
            Verbosity::Standard => "Verbosity::Standard",
            Verbosity::Quiet => "Verbosity::Quiet",
            Verbosity::ErrorsOnly => "Verbosity::ErrorsOnly",
        };
        write!(f, "{}", level_str)
    }
}
