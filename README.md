# libprettylogger
A Rust logger library, focused on stability and customizability.


## Table of Contents
* [TL;DR](#tldr)
* [Installation](#installation)
    * [With `Cargo`](#installation_with-cargo)
    * [Manually](#installation_manually)
* [The Log Anatomy](#the-log-anatomy)
* [The Logger](#the-logger)
    * [Logging Methods](#the-logger_logging-methods)
    * [Setters](#the-logger_setters)
    * [Other Methods](#the-logger_other-methods)


<a name="tldr"></a>
## TL;DR
**Installation**:
* `git clone https://github.com/tpaau-17DB/libprettylogger.git`
* `cd libprettylogger`
* `cargo build --release`

**Use the `.rlib` file in your project**:
* Move `target/release/libprettylogger.rlib` to your project directory
* Include it in `Cargo.toml`:
```toml
[dependencies]
prettylogger = "/path/to/libprettylogger.rlib"
```

<!--**Installation**:-->
<!--Update your `Cargo.toml`:-->
<!--```toml-->
<!--[dependencies]-->
<!--prettylogger = "0.1.0"-->
<!--```-->

**Using in your project**:
<!--Make sure this matches the example from lib.rs-->
```rust
// Include stuff from the library:
use prettylogger::logging::Logger;
use prettylogger::filtering::Verbosity;

// A `Logger struct with default configuration`
let mut logger = Logger::default();///

// Configure `Logger` to your liking
logger.set_verbosity(Verbosity::All); // Don't suppress any log messages

// Print logs: 
logger.debug("A debug message!");
logger.info("Info message!");
logger.warning("A warning!");
logger.error("An error!");
logger.fatal("A fatal error!");
```


<a name="installation"></a>
## Installation
<a name="installation_with-cargo"></a>
### With `Cargo`
**CURRENTLY NOT SUPPORTED**
To install the library with `cargo`, run:
```
cargo install prettylogger
```

To use it in your own project, add this to your `Cargo.toml`:
```toml
[dependencies]
prettylogger = "0.1.0"
```

<a name="installation_manually"></a>
### Manual installation
Currently, the only way to install the library is to download it from its 
repository and build it manually with cargo:
```
cargo build --release
```
This will produce a `libprettylogger.rlib` in the `target/release/` directory.
You can copy this file to your project's directory and include it in `Cargo.toml`:
```toml
[dependencies]
prettylogger = "/path/to/your/repo/lib/libprettylogger.rlib"
```


<a name="the-log-anatomy"></a>
## The Log Anatomy
A log consists of several headers:
* **Log header** -> Determined by the type of the log (eg. debug, info, warning)
* **Timestamp** -> Contains the date and time the log was created
* **Message** -> The actual log message

Here is a log message with all its parts highlighted:
```
[ DEBUG 21:52:37 An example debug message ]
  ^^^^^ ^^^^^^^^ ^^^^^^^^^^^^^^^^^^^^^^^^
  |     |        |
  |     |        |
  |     |        the message
  |     |
  |     timestamp
  |
  log header
```
This specific effect was achieved by setting the datetime format to `%H:%M:%S`,
log format to `[ %h %d %m ]` and the debug header to `DEBUG`.


<a name="the-logger"></a>
## The Logger
The `Logger` struct is the **core** of the entire project.
This is what you are going to use when you want to print a log, set filtering
rules or modify log formatting. 
All of its fields are private, only allowing for configuration via **setters**.

Creating a `Logger` struct with default configuration:
```rust
let mut logger = Logger::default();
```

<a name="the-logger_logging-methods"></a>
### Logging Methods:
* `debug(&mut self, message: &str)` -> Prints a **debug message**.
* `info(&mut self, message: &str)` -> Prints **info message**.
* `warning(&mut self, message: &str)` -> Prints a **warning**.
* `error(&mut self, message: &str)` -> Prints an **error**.
* `fatal(&mut self, message: &str)` -> Prints a **fatal error**.
BTW, `debug`, `info` and `warning` methods have their variants that bypass
filtering:
* `debug_no_filtering(&mut self, message: &str)` -> Prints a **debug message**,
bypasses filtering.
* `info_no_filtering(&mut self, message: &str)` -> Prints **info message**,
bypasses filtering.
* `warning_no_filtering(&mut self, message: &str)` -> Prints a **warning**,
bypasses filtering.
Note that`error` and `fatal` methods don't have `_no_filtering` variants.
That is because errors **can't be suppressed**.

<a name="the-logger_setters"></a>
### Setters:
**Log filtering** (see [this](#log-filtering)):
* `set_verbosity(&mut self, verbosity: Verbosity)` -> Sets the `Logger` verbosity. The `Verbosity` `enum` is declared in `prettylogger::filtering`.
* `toggle_log_filtering(&mut self, enabled: bool)` -> Toggles log filtering.

**Log format** (see [this](#the-log-anatomy)):
* `toggle_log_header_color(&mut self, enabled: bool)` -> Toggles log header color,
same as setting all the log header colors to `Color::None`.
* `set_debug/info/warning/error/fatal_header(&mut self, header: &str)` -> Sets
the log header for different log types (debug, info, warning, error, fatal).
* `set_debug/info/warning/error/fatal_color(&mut self, color: Color)` -> Sets
the log header color for different log types. The `Color` `enum` is declared in
`prettylogger::colors`.
* `set_datetime_format(&mut self, format: &str)` -> Sets the timestamp format. 
* `set_log_format(&mut self, format: &str)` -> Sets the log format.

**File logging** (see [this](#file-logging)):
* `set_log_file_path(&mut self, path: &str)` -> Sets the log file path.
* `toggle_file_logging(&ut self, enabled: bool)` -> Toggles file logging.
* `set_max_log_buffer_size(&mut self, size: usize)` -> Sets the maximum size
of the log buffer. When log buffer exceeds this limit, it gets flushed.
* `toggle_log_file_lock(&mut self, enabled: bool)` -> Toggles log file lock
used to avoid race conditions.

<a name="the-logger_other-methods"></a>
### Other Methods:
* `format_log(&mut self, log: &LogStruct)` -> Returns a formatted log based on the
`LogStruct` and `Logger` configuration. The `LogStruct` is declared in
`prettylogger::logging`.
* `flush(&mut self)` -> Flushes the log buffer.


<a name="log-filtering"></a>
## Log filtering
Lorem ipsum


<a name="file-logging"></a>
## File Logging
Lorem ipsum
