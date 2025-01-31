use crate::logging::*;
use lazy_static::lazy_static;
use std::{
    sync::Mutex,
    fmt,
};

#[derive(Clone)]
pub enum LogLevel {
    All = 0,
    Standard = 1,
    Quiet = 2,
    ErrorsOnly = 3,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let level_str = match *self {
            LogLevel::All => "LogLevel::All",
            LogLevel::Standard => "LogLevel::Standard",
            LogLevel::Quiet => "LogLevel::Quiet",
            LogLevel::ErrorsOnly => "LogLevel::ErrorsOnly",
        };
        write!(f, "{}", level_str)
    }
}

lazy_static! {
    pub static ref LOG_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::All);
}

lazy_static! {
    pub static ref FILTERING_ENABLED: Mutex<bool> = Mutex::new(true);
}

pub fn filter_log(log_type: LogType) -> bool {
    return !*FILTERING_ENABLED.lock().unwrap()
        || ((log_type as i32) < LOG_LEVEL.lock().unwrap().clone() as i32)

}
