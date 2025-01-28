use crate::utils::{
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

pub fn debug(message: &String) {
    if !filter_log(LogType::Debug)
    {
        return;
    }
    let log = LogStruct {
        message,
        log_type: LogType::Debug,
    };
    print_log(&log);
}

pub fn debug_no_filtering(message: &String) {
    let log = LogStruct {
        message,
        log_type: LogType::Debug,
    };
    print_log(&log);
}


pub fn info(message: &String) {
    if !filter_log(LogType::Info)
    {
        return;
    }
    let log = LogStruct {
        message,
        log_type: LogType::Info,
    };
    print_log(&log);
}

pub fn info_no_filtering(message: &String) {
    let log = LogStruct {
        message,
        log_type: LogType::Info,
    };
    print_log(&log);
}

pub fn warn(message: &String) {
    if !filter_log(LogType::Warning)
    {
        return;
    }
    let log = LogStruct {
        message,
        log_type: LogType::Warning,
    };
    print_log(&log);
}

pub fn warn_no_filtering(message: &String) {
    let log = LogStruct {
        message,
        log_type: LogType::Warning,
    };
    print_log(&log);
}

pub fn err(message: &String) {
    // error messages cant get suppressed
    let log = LogStruct {
        message,
        log_type: LogType::Error,
    };
    print_log(&log);
}

pub fn fatal(message: &String) {
    // error messages cant get suppressed
    let log = LogStruct {
        message,
        log_type: LogType::FatalError,
    };
    print_log(&log);
}
