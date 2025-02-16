/// A highly customizable logger library.
///
/// # How to use:
/// ```
/// // Include stuff from the library:
/// use prettylogger::logging::Logger;
/// use prettylogger::filtering::Verbosity;
///
/// // A `Logger` struct with default configuration
/// let mut logger = Logger::default();///
///
/// // Configure `Logger` to your liking
/// logger.set_verbosity(Verbosity::All); // Don't suppress any log messages
///
/// // Print logs: 
/// logger.debug("A debug message!");
/// logger.info("Info message!");
/// logger.warning("A warning!");
/// logger.error("An error!");
/// logger.fatal("A fatal error!");
/// ```

pub mod logging;
pub mod colors;
pub mod filtering;
mod setters;
mod json;
mod fileio;
