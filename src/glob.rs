use std::sync::{
    LazyLock,
    RwLock
};

use crate::Logger;

/// Global `Logger` struct that can be used with the `debug!`, `info!`, `warn!`,
/// `err!`, and `fatal!` macros.
///
/// > **Note**
/// > Do not use any log macros when you are writing to the global logger, it
/// > will cause your thread to lock.
///
/// This will block the thread:
/// ```no_run
/// # use prettylogger::{info, glob::LOGGER};
/// // Get write access to the logger
/// let mut logger = LOGGER.write().unwrap();
///
/// // Trying to use logging macro blocks the thread
/// info!("This will never be shown!");
/// ````
///
/// # Examples
///
/// Using global logging:
/// ```
/// # use prettylogger::{debug, info, warn, err, fatal};
/// debug!("This is a debug message!");
/// info!("This is an info message!");
/// warn!("This is a warning!");
/// err!("This is an error!");
/// fatal!("This is a fatal error!");
/// ```
///
/// Configuring the global logger:
/// ```
/// # use prettylogger::{
/// #     config::Verbosity,
/// #     glob::LOGGER,
/// #     debug
/// # };
/// // Configure logger
/// let mut logger = LOGGER.write().unwrap();
/// logger.set_verbosity(Verbosity::All);
///
/// // Drop the logger so a read lock can be acquired
/// drop(logger);
///
/// // Then print a message
/// debug!("This should be shown!");
/// ```
pub static LOGGER: LazyLock<RwLock<Logger>>
    = LazyLock::new(|| RwLock::new(Logger::default()));


/// Prints a debug message using the global `Logger` instance.
///
/// Not to be confused with the `dbg!` macro.
///
/// > **Warning!**
/// > This macro will block if any thread holds write access to the global
/// > logger.
///
/// # Panics
/// Panics if the global logger's lock is poisoned.
///
/// # Examples
///
/// Using the `debug!` macro:
/// ```
/// use prettylogger::debug;
/// let name = String::from("world");
/// debug!("Hello, {name}!");
/// ```
#[macro_export]
macro_rules! debug {
    ($($t:tt)*) => {{
        use $crate::glob::LOGGER;
        LOGGER
            .read()
            .unwrap()
            .debug(&format!($($t)*));
    }};
}

/// Prints an info message using the global `Logger` instance.
///
/// > **Warning!**
/// > This macro will block if any thread holds write access to the global
/// > logger.
///
/// # Panics
/// Panics if the global logger's lock is poisoned.
///
/// # Examples
///
/// Using the `info!` macro:
/// ```
/// use prettylogger::info;
/// let name = String::from("world");
/// info!("Hello, {name}!");
/// ```
#[macro_export]
macro_rules! info {
    ($($t:tt)*) => {{
        use $crate::glob::LOGGER;
        LOGGER
            .read()
            .unwrap()
            .info(&format!($($t)*));
    }};
}

/// Prints a warning using the global `Logger` instance.
///
/// > **Warning!**
/// > This macro will block if any thread holds write access to the global
/// > logger.
///
/// # Panics
/// Panics if the global logger's lock is poisoned.
///
/// # Examples
///
/// Using the `warn!` macro:
/// ```
/// use prettylogger::warn;
/// let name = String::from("world");
/// warn!("Hello, {name}!");
/// ```
#[macro_export]
macro_rules! warn {
    ($($t:tt)*) => {{
        use $crate::glob::LOGGER;
        LOGGER
            .read()
            .unwrap()
            .warning(&format!($($t)*));
    }};
}

/// Prints an error using the global `Logger` instance.
///
/// > **Warning!**
/// > This macro will block if any thread holds write access to the global
/// > logger.
///
/// # Panics
/// Panics if the global logger's lock is poisoned.
///
/// # Examples
///
/// Using the `err!` macro:
/// ```
/// use prettylogger::err;
/// let name = String::from("world");
/// err!("Hello, {name}!");
/// ```
#[macro_export]
macro_rules! err {
    ($($t:tt)*) => {{
        use $crate::glob::LOGGER;
        LOGGER
            .read()
            .unwrap()
            .error(&format!($($t)*));
    }};
}

/// Prints a fatal error using the global `Logger` instance.
///
/// > **Warning!**
/// > This macro will block if any thread holds write access to the global
/// > logger.
///
/// # Panics
/// Panics if the global logger's lock is poisoned.
///
/// # Examples
///
/// Using the `fatal!` macro:
/// ```
/// use prettylogger::fatal;
/// let name = String::from("world");
/// fatal!("Hello, {name}!");
/// ```
#[macro_export]
macro_rules! fatal {
    ($($t:tt)*) => {{
        use $crate::glob::LOGGER;
        LOGGER
            .read()
            .unwrap()
            .fatal(&format!($($t)*));
    }};
}
