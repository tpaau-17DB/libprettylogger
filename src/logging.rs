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

fn print_debug(message: &String, ignore_filtering: Option<bool>) {
    match ignore_filtering {
        Some(false) | None => {
            if !filter_log(LogType::Debug)
            {
                return;
            }
            let log = LogStruct {
                message,
                log_type: LogType::Debug,
            };
            print_log(&log);
        },
        Some(true) => {
            let log = LogStruct {
                message,
                log_type: LogType::Debug,
            };
            print_log(&log);
        },
    }
}

fn print_info(message: &String, ignore_filtering: Option<bool>) {
    match ignore_filtering {
        Some(false) | None => {
        },
        Some(true) => {
        },
    }
}

fn print_warning(message: &String, ignore_filtering: Option<bool>) {
    match ignore_filtering {
        Some(false) | None => {
        },
        Some(true) => {
        },
    }
}

fn print_error(message: &String, ignore_filtering: Option<bool>) {
    match ignore_filtering {
        Some(false) | None => {
        },
        Some(true) => {
        },
    }
}

fn print_fatal_error(message: &String, ignore_filtering: Option<bool>) {
    match ignore_filtering {
        Some(false) | None => {
        },
        Some(true) => {
        },
    }
}
