use std::sync::{
    LazyLock,
    RwLock
};

use crate::Logger;

/// Global `Logger` struct that can be used with the `debg!`, `info!`, `warn!`,
/// `err!`, and `fatal!` macros.
pub static LOGGER: LazyLock<RwLock<Logger>>
    = LazyLock::new(|| RwLock::new(Logger::default()));


/// Prints a debug message using the global `Logger` instance.
///
/// Not to be confused with the `dbg!` macro.
///
/// > **Warning!**
/// > This macro will block if any thread holds read access to the global
/// > logger’s RW lock.
///
/// # Panics
/// Panics if the global logger's lock is poisoned.
///
/// # Examples
/// ```rust
/// use prettylogger::debg;
/// let name = String::from("world");
/// debg!("Hello, {name}!");
/// ```
#[macro_export]
macro_rules! debg {
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
/// > This macro will block if any thread holds read access to the global
/// > logger’s RW lock.
///
/// # Panics
/// Panics if the global logger's lock is poisoned.
///
/// # Examples
/// ```rust
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

/// Prints a warning message using the global `Logger` instance.
///
/// > **Warning!**
/// > This macro will block if any thread holds read access to the global
/// > logger’s RW lock.
///
/// # Panics
/// Panics if the global logger's lock is poisoned.
///
/// # Examples
/// ```rust
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

/// Prints an error message using the global `Logger` instance.
///
/// > **Warning!**
/// > This macro will block if any thread holds read access to the global
/// > logger’s RW lock.
///
/// # Panics
/// Panics if the global logger's lock is poisoned.
///
/// # Examples
/// ```rust
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

/// Prints a fatal error message using the global `Logger` instance.
///
/// > **Warning!**
/// > This macro will block if any thread holds read access to the global
/// > logger’s RW lock.
///
/// # Panics
/// Panics if the global logger's lock is poisoned.
///
/// # Examples
/// ```rust
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
