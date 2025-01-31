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
    };

    #[test]
    fn test_filtering() {
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
    fn test_colors() {
        if colorify(&"a".to_string(), Color::Red) != "\x1b[31ma\x1b[0m"
        {
            panic!("Failed to colorify a string!");
        }
    }
}
