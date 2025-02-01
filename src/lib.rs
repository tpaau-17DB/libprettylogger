pub mod logging;
pub mod setters;
pub mod colors;
mod filtering;
mod formatting;
mod printing;

#[cfg(test)]
mod tests {
    use crate::{
        logging::*,
        filtering::*,
        setters::*,
        colors::*,
        formatting::*,
    };

    #[test]
    fn test_log_filtering() {
        // Check is filtering is working properly.
        toggle_log_filtering(true);
        set_verbosity(LogLevel::ErrorsOnly);
        if !filter_log(LogType::Debug) {
            panic!("A debug log should get filtered for verbosity set to: {}", LogLevel::ErrorsOnly);
        }
        if !filter_log(LogType::Info) {
            panic!("An informative log should get filtered for verbosity set to: {}", LogLevel::ErrorsOnly);
        }
        if !filter_log(LogType::Warning) {
            panic!("A warning log should get filtered for verbosity set to: {}", LogLevel::ErrorsOnly);
        }

        set_verbosity(LogLevel::Quiet);
        if !filter_log(LogType::Debug) {
            panic!("A debug log should get filtered for verbosity set to: {}", LogLevel::ErrorsOnly);
        }
        if !filter_log(LogType::Info) {
            panic!("An informative log should get filtered for verbosity set to: {}", LogLevel::ErrorsOnly);
        }
        if filter_log(LogType::Warning) {
            panic!("A warning log not should get filtered for verbosity set to: {}", LogLevel::ErrorsOnly);
        }

        set_verbosity(LogLevel::Standard);
        if !filter_log(LogType::Debug) {
            panic!("A debug log should get filtered for verbosity set to: {}", LogLevel::ErrorsOnly);
        }
        if filter_log(LogType::Info) {
            panic!("An informative log should not get filtered for verbosity set to: {}", LogLevel::ErrorsOnly);
        }
        if filter_log(LogType::Warning) {
            panic!("A warning log not should get filtered for verbosity set to: {}", LogLevel::ErrorsOnly);
        }

        set_verbosity(LogLevel::All);
        if filter_log(LogType::Debug) {
            panic!("A debug log should not get filtered for verbosity set to: {}", LogLevel::ErrorsOnly);
        }
        if filter_log(LogType::Info) {
            panic!("An informative log should not get filtered for verbosity set to: {}", LogLevel::ErrorsOnly);
        }
        if filter_log(LogType::Warning) {
            panic!("A warning log not should get filtered for verbosity set to: {}", LogLevel::ErrorsOnly);
        }

        set_verbosity(LogLevel::All);
        toggle_log_filtering(true);
        if filter_log(LogType::Debug) {
            panic!("A debug log should not get filtered for verbosity set to: {}", LogLevel::ErrorsOnly);
        }
        if filter_log(LogType::Info) {
            panic!("An informative log should not get filtered for verbosity set to: {}", LogLevel::ErrorsOnly);
        }
        if filter_log(LogType::Warning) {
            panic!("A warning log not should get filtered for verbosity set to: {}", LogLevel::ErrorsOnly);
        }
    }

    #[test]
    fn test_log_colors() {
        // Test if colorify works
        if colorify("a", Color::Red) != "\x1b[31ma\x1b[0m"
        {
            panic!("Failed to colorify a string!");
        }
    }

    #[test]
    fn test_log_headers() {
        // Test if header format setting works
        let header = "askljdfh";

        set_debug_header(header);
        if get_header(&LogType::Debug) != header {
            panic!("Debug headers do not match!");
        }
        set_info_header(header);
        if get_header(&LogType::Info) != header {
            panic!("Debug headers do not match!");
        }
        set_warning_header(header);
        if get_header(&LogType::Warning) != header {
            panic!("Debug headers do not match!");
        }
        set_error_header(header);
        if get_header(&LogType::Error) != header {
            panic!("Debug headers do not match!");
        }
        set_fatal_header(header);
        if get_header(&LogType::FatalError) != header {
            panic!("Debug headers do not match!");
        }
    }

    #[test]
    fn test_datetime_header() {
        // Test if datetime header format is parsed correctly
        if set_datetime_header_format("[{}]") == 1 {
            panic!("Error while setting datetime header format!");
        }
        if set_datetime_header_format("{}") == 1 {
            panic!("Error while setting datetime header format!");
        }
        if set_datetime_header_format("[]") == 0 {
            panic!("Format invalid, but no error thrown!");
        }
    }
}
