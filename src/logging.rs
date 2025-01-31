use crate::{
    filtering::*,
    printing::*,
};

pub enum LogType {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
    FatalError = 4,
}

pub struct LogStruct<'a> {
    pub message: &'a String,
    pub log_type: LogType,
}

/// Prints a **debug log**
pub fn debug(message: &String) {
    if filter_log(LogType::Debug)
    {
        return;
    }
    let log = LogStruct {
        message,
        log_type: LogType::Debug,
    };
    print_log(&log);
}

/// Prints a **debug log**, completely bypasses any filtering
pub fn debug_no_filtering(message: &String) {
    let log = LogStruct {
        message,
        log_type: LogType::Debug,
    };
    print_log(&log);
}


/// Prints an **informative log**
pub fn info(message: &String) {
    if filter_log(LogType::Info)
    {
        return;
    }
    let log = LogStruct {
        message,
        log_type: LogType::Info,
    };
    print_log(&log);
}

/// Prints an **informative log**, completely bypasses filtering
pub fn info_no_filtering(message: &String) {
    let log = LogStruct {
        message,
        log_type: LogType::Info,
    };
    print_log(&log);
}


/// Prints a **warning**
pub fn warn(message: &String) {
    if filter_log(LogType::Warning)
    {
        return;
    }
    let log = LogStruct {
        message,
        log_type: LogType::Warning,
    };
    print_log(&log);
}

/// Prints a **warning**, completely bypasses filtering
pub fn warn_no_filtering(message: &String) {
    let log = LogStruct {
        message,
        log_type: LogType::Warning,
    };
    print_log(&log);
}


/// Prints an **error**
pub fn err(message: &String) {
    // error messages cant get suppressed
    let log = LogStruct {
        message,
        log_type: LogType::Error,
    };
    print_log(&log);
}


/// Prints a **fatal error**
pub fn fatal(message: &String) {
    // error messages cant get suppressed
    let log = LogStruct {
        message,
        log_type: LogType::FatalError,
    };
    print_log(&log);
}
