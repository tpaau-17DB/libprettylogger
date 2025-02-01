use crate::{
    filtering::*,
    colors::*,
    formatting::*,
    logging::*,
};

// FILTERING

/// Lets you set **verbosity** to desired level
/// * `All`        -> Don't filter any logs
/// * `Standard`   -> Just filter debug logs
/// * `Quiet`      -> Only let warnings and errors to be displayed
/// * `ErrorsOnly` -> I'm not gonna explain this one
pub fn set_verbosity(log_level: LogLevel) {
    let mut ll = LOG_LEVEL.lock().unwrap();
    *ll = log_level;
}

/// Lets you toggle log filtering
/// * **true**  -> logs will get filtered based on verbosity
/// * **false** -> log filtering will be disabled globally
pub fn toggle_log_filtering(enabled: bool) {
    let mut filtering_enabled =  FILTERING_ENABLED.lock().unwrap();
    *filtering_enabled = enabled;
}

// FORMATTING && COLORS

/// Toggles colored log headers
/// * `true`  -> Log headers will have colors 
/// * `false` -> No colors :(
pub fn toggle_log_color(enabled: bool) {
    let mut m = USE_COLOR.lock().unwrap();
    *m = enabled;
}

/// Sets **debug log header** color
pub fn set_debug_color(color: Color) {
    let mut m = DEBUG_COLOR.lock().unwrap();
    *m = color;
}

/// Sets **info log header** color
pub fn set_info_color(color: Color) {
    let mut m = INFO_COLOR.lock().unwrap();
    *m = color;
}

/// Sets **warning header** color
pub fn set_warning_color(color: Color) {
    let mut m = WARN_COLOR.lock().unwrap();
    *m = color;
}

/// Sets **error header** color
pub fn set_error_color(color: Color) {
    let mut m = ERR_COLOR.lock().unwrap();
    *m = color;
}

/// Sets **fatal error header** color
pub fn set_fatal_color(color: Color) {
    let mut m = FATAL_COLOR.lock().unwrap();
    *m = color;
}

/// Sets **debug log header** format
pub fn set_debug_header(format: &str) {
    let mut m  = DEBUG_HEADER.lock().unwrap();
    *m = format.to_string();
}

/// Sets **info log header** format
pub fn set_info_header(format: &str) {
    let mut m  = INFO_HEADER.lock().unwrap();
    *m = format.to_string();
}

/// Sets **warning header** format
pub fn set_warning_header(format: &str) {
    let mut m  = WARN_HEADER.lock().unwrap();
    *m = format.to_string();
}

/// Sets **error header** format
pub fn set_error_header(format: &str) {
    let mut m  = ERR_HEADER.lock().unwrap();
    *m = format.to_string();
}

/// Sets **fatal error header** format
pub fn set_fatal_header(format: &str) {
    let mut m  = FATAL_HEADER.lock().unwrap();
    *m = format.to_string();
}

/// # Sets datetime header format
///
/// Note that this function sets the **header format** alone, 
/// not the **datetime format**!
///
/// Format must contain the `{}` string to tell logger where
/// to put datetime string.
///
/// For example, if you set the format to `<d>{}</d>`, 
/// a datetime header will be displayed like this:
/// **<d>YYYY:MM:DD:HH:mm</d>**
///
/// # Return values:
/// * `0` -> Success
/// * `1` -> Invalid format
pub fn set_datetime_header_format(format: &str) -> i32 {
    let mut open_index = None;
    let mut close_index = None;

    for (i, c) in format.char_indices() {
        if c == '{' {
            open_index = Some(i);
        }
        else if c == '}' && open_index.unwrap() == i - 1 {
            close_index = Some(i);
            break;
        }
        else {
            open_index = None;
            close_index = None;
        }
    }

    if open_index.is_none() || close_index.is_none() 
    {
        error(&format!("Invalid datetime header format: '{format}'"));
        return 1;
    }
    else {
        let opening = &format[0..open_index.unwrap()];
        let closing = &format[close_index.unwrap() + 1..format.len()];

        let mut m = DATETIME_HEADER_LEFT_BRACKET.lock().unwrap();
        *m = opening.to_string();
        m = DATETIME_HEADER_RIGHT_BRACKET.lock().unwrap();
        *m = closing.to_string();
    }

    return 0;
}

/// Toggles datetime header for every log
/// * `true`  -> Every log will have a datetime header
/// * `false` -> No datetime headers
pub fn toggle_show_datetime(enabled: bool) {
    let mut m = SHOW_DATETIME.lock().unwrap();
    *m = enabled;
}
