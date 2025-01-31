use crate::{
    filtering::*,
    colors::*,
    formatting::*,
};

// FILTERING
//
/// Lets you set **verbosity** to desired level
///
/// * **All**        -> Don't filter any logs
/// * **Standard**   -> Just filter debug logs
/// * **Quiet**      -> Only let warnings and errors to be displayed
/// * **ErrorsOnly** -> I'm not gonna explain this one
pub fn set_verbosity(log_level: LogLevel) {
    let mut ll = LOG_LEVEL.lock().unwrap();
    *ll = log_level;
}

/// Lets you toggle log filtering
///
/// * **true**  -> logs will get filtered based on verbosity
/// * **false** -> log filtering will be disabled globally
pub fn toggle_log_filtering(enabled: bool) {
    let mut filtering_enabled =  FILTERING_ENABLED.lock().unwrap();
    *filtering_enabled = enabled;
}

// FORMATTING && COLORS

/// Toggles colored log headers
///
/// * **true**  -> Log headers will be colored
/// * **false** -> No color :(
pub fn log_color_enabled(enabled: bool) {
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
