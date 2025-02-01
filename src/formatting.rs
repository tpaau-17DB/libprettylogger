use lazy_static::lazy_static;
use std::sync::Mutex;
use chrono::prelude::*;
use crate::logging::*;
use crate::colors::*;

// Log headers
lazy_static! {
    pub static ref DEBUG_HEADER: Mutex<String> = Mutex::new("[DBG]".to_string());
}
lazy_static! {
    pub static ref INFO_HEADER: Mutex<String> = Mutex::new("[INF]".to_string());
}
lazy_static! {
    pub static ref WARN_HEADER: Mutex<String> = Mutex::new("[WAR]".to_string());
}
lazy_static! {
    pub static ref ERR_HEADER: Mutex<String> = Mutex::new("[ERR]".to_string());
}
lazy_static! {
    pub static ref FATAL_HEADER: Mutex<String> = Mutex::new("[FATAL]".to_string());
}


// Log colors
lazy_static! {
    pub static ref DEBUG_COLOR: Mutex<Color> = Mutex::new(Color::Blue);
}
lazy_static! {
    pub static ref INFO_COLOR: Mutex<Color> = Mutex::new(Color::Green);
}
lazy_static! {
    pub static ref WARN_COLOR: Mutex<Color> = Mutex::new(Color::Yellow);
}
lazy_static! {
    pub static ref ERR_COLOR: Mutex<Color> = Mutex::new(Color::Red);
}
lazy_static! {
    pub static ref FATAL_COLOR: Mutex<Color> = Mutex::new(Color::Magenta);
}

lazy_static! {
    pub static ref DATETIME_HEADER_LEFT_BRACKET: Mutex<String> = 
        Mutex::new("[".to_string());
}
lazy_static! {
    pub static ref DATETIME_HEADER_RIGHT_BRACKET: Mutex<String> = 
        Mutex::new("[".to_string());
}

lazy_static! {
    pub static ref SHOW_DATETIME: Mutex<bool> = Mutex::new(false);
}

fn get_datetime_header() -> String {
    let time = Local::now();
    let right = DATETIME_HEADER_RIGHT_BRACKET.lock().unwrap(); 
    let left = DATETIME_HEADER_LEFT_BRACKET.lock().unwrap(); 
    return format!("{left}{time}{right}");
}

pub fn get_header(log_type: &LogType) -> String {
    match log_type {
        LogType::Debug => { DEBUG_HEADER.lock().unwrap().to_string() }
        LogType::Info => { INFO_HEADER.lock().unwrap().to_string() }
        LogType::Warning => { WARN_HEADER.lock().unwrap().to_string() }
        LogType::Error => { ERR_HEADER.lock().unwrap().to_string() }
        LogType::FatalError => { FATAL_HEADER.lock().unwrap().to_string() }
    }
}

fn get_color(log_type: &LogType) -> Color {
    match log_type {
        LogType::Debug => { DEBUG_COLOR.lock().unwrap().clone() }
        LogType::Info => { INFO_COLOR.lock().unwrap().clone() }
        LogType::Warning => { WARN_COLOR.lock().unwrap().clone() }
        LogType::Error => { ERR_COLOR.lock().unwrap().clone() }
        LogType::FatalError => { FATAL_COLOR.lock().unwrap().clone() }
    }
}

pub fn format_log(log: &LogStruct) -> String {
    if !*SHOW_DATETIME.lock().unwrap() {
        return format!("{} {}\n",
            colorify(&get_header(&log.log_type), get_color(&log.log_type)),
            log.message);
    }
    else {
        return format!("{} {} {}\n",
            colorify(&get_header(&log.log_type), get_color(&log.log_type)),
            get_datetime_header(),
            log.message);
    }
}
