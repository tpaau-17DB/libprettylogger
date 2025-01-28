use crate::logging::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Clone)]
pub enum LogLevel {
    All = 0,
    Standard = 1,
    Quier = 2,
    ErrorsOnly = 3,
}

lazy_static! {
    static ref LOG_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::Standard);
}

pub fn filter_log(log_type: LogType) -> bool {
    return log_type as i32 > LOG_LEVEL.lock().unwrap().clone() as i32
}
