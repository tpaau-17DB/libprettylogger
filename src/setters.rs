use crate::filtering::{LOG_LEVEL, LogLevel};

pub fn set_verbosity(log_level: LogLevel) {
    let mut ll = LOG_LEVEL.lock().unwrap();
    *ll = log_level;
}
