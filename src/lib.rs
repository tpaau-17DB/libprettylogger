pub mod logging;
pub mod setters;
pub mod colors;
mod filtering;
mod formatting;
mod printing;

#[cfg(test)]
mod tests {
    use crate::{logging::*, filtering::*, setters::*};

    #[test]
    fn printing_runtime_errors() {
        debug(&"Debug log.".to_string());
        info(&"Informative log.".to_string());
        warn(&"A warning.".to_string());
        err(&"An error.".to_string());
        fatal(&"A fatal error.".to_string());
    }

    #[test]
    fn setting_runtime_errors() {
        set_verbosity(LogLevel::ErrorsOnly);
        set_verbosity(LogLevel::Quiet);
        set_verbosity(LogLevel::Standard);
        set_verbosity(LogLevel::All);
    }

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
    }
}
