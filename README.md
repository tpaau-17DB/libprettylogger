# libprettylogger
A highly customizable logger library written in Rust.

![CI Ubuntu](https://img.shields.io/github/actions/workflow/status/tpaau-17DB/libprettylogger/Ubuntu.yml?branch=main)
![Crates.io](https://img.shields.io/crates/v/libprettylogger.svg)


## Table of Contents
* [TL;DR](#tldr)
* [Installation](#installation)
* [Log Format](#log-format)
* [The Logger](#the-logger)
    * [Constructors](#the-logger_constructors)
    * [Logging Methods](#the-logger_logging-methods)
    * [Setters](#the-logger_setters)
    * [Other Methods](#the-logger_other-methods)
* [Log Filtering](#log-filtering)
* [File Logging](#file-logging)
    * [Automatic Log File Flushing](#file-logging_automatic-log-buffer-flushing)
    * [Locking the Log File](#file-logging_locking-the-log-file)
* [Logger Templates](#logger-templates)


<a name="tldr"></a>
## TL;DR
**Install the library**:
```
cargo add libprettylogger
```

**Include the library in your project**:
<!--Make sure this matches the example from lib.rs-->
```rust
// Include stuff from the library:
use prettylogger::logging::Logger;
use prettylogger::filtering::Verbosity;

// A `Logger` struct with default configuration
let mut logger = Logger::default();

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
To install the library with `cargo`, run:
```
cargo add libprettylogger
```

Or add this to your `Cargo.toml`:
```toml
[dependencies]
libprettylogger = "1.0.0"
```


<a name="the-logger"></a>
## The Logger
The `Logger` struct is the **core** of the entire project.
This is what you are going to use when you want to print a log, set filtering
rules or modify log formatting.
All of it's fields are private, only allowing for modification via setters.

Creating a `Logger` struct with default configuration:
```rust
let mut logger = Logger::default();
```

<a name="the-logger_constructors"></a>
### Constructors:
* `default()` **→** `Logger` with default configuration.
* `from_template(path: &str)` **→** Deserializes `Logger` from a JSON template
file (see [this](#logger-templates)).

<a name="the-logger_logging-methods"></a>
### Logging Methods:
* `debug(message: &str)` ****→**** Prints a **debug message**.
* `info(message: &str)` **→** Prints **info message**.
* `warning(message: &str)` **→** Prints a **warning**.
* `error(message: &str)` **→** Prints an **error**.
* `fatal(message: &str)` **→** Prints a **fatal error**.

**BTW**, `debug`, `info` and `warning` methods have their variants that bypass
filtering:
* `debug_no_filtering(message: &str)` **→** Prints a **debug message**,
bypasses filtering.
* `info_no_filtering(message: &str)` **→** Prints **info message**, bypasses
filtering.
* `warning_no_filtering(message: &str)` **→** Prints a **warning**, bypasses
filtering.

Note that `error` and `fatal` methods don't have `_no_filtering` variants.
This is because errors **can't be suppressed**.

<a name="the-logger_setters"></a>
### Setters:
**Log filtering** (see [this](#log-filtering)):
* `set_verbosity(verbosity: Verbosity)` **→** Sets the `Logger`
verbosity.
* `toggle_log_filtering(enabled: bool)` **→** Toggles log filtering.

**Log formatting** (see [this](#log-format)):
* `toggle_log_header_color(enabled: bool)` **→** Toggles log type
header color, same as setting all the log type header colors to `Color::None`.
* `set_debug/info/warning/error/fatal_header(header: &str)` **→** Sets
the log type header for different log types (debug, info, warning, error, fatal).
* `set_debug/info/warning/error/fatal_color(color: Color)` **→** Sets
the log type header color for different log types. The `Color` enum is declared in
`prettylogger::colors`.
* `set_datetime_format(format: &str)` **→** Sets the timestamp format.
* `set_log_format(format: &str)` **→** Sets the log format.

**File logging** (see [this](#file-logging)):
* `set_log_file_path(path: &str)` **→** Sets the log file path.
* `toggle_file_logging(enabled: bool)` **→** Toggles file logging.
* `set_max_log_buffer_size(size: usize)` **→** Sets the maximum size
of the log buffer. When log buffer exceeds this limit, it gets flushed.
* `toggle_log_file_lock(enabled: bool)` **→** Toggles log file lock
used to avoid race conditions.

<a name="the-logger_other-methods"></a>
### Other Methods:
* `format_log(log: &LogStruct)` **→** Returns a formatted log based on
the `LogStruct` and `Logger` configuration. The `LogStruct` is declared in
`prettylogger::logging`.
* `flush()` **→** Flushes the log buffer.
* `save_template(path: &str)` **→** Serializes `Logger` into a JSON
template file. (see [this](#logger-templates))


<a name="log-format"></a>
## Log Format
A log consists of several headers:
* **Log Type** **→** The type of the log (debug, info, warning etc.)
* **Timestamp** **→** Contains the date and time the log was created
* **Message** **→** The actual log message

Here is a log message with all it's parts marked:
```
[ DEBUG 21:52:37 An example debug message ]
  ^^^^^ ^^^^^^^^ ^^^^^^^^^^^^^^^^^^^^^^^^
  |     |        |
  |     |        the message
  |     timestamp
  log type
```
This specific effect was achieved by setting the datetime format to `%H:%M:%S`,
log format to `[ %h %d %m ]` and the debug log type header to `DEBUG`.

You can use this method to set the datetime format of a `Logger`:
```rust
logger.set_datetime_format("%H:%M:%S");
```

Log format can be customized using:
```rust
logger.set_log_format("[ %h %d %m ]");
```
**Note**: The `%m` placeholder is mandatory and you will get an error unless you
include it in your format string.

And log type headers can be customized with their corresponding methods:
```rust
logger.set_debug_header("DEBUG");
logger.set_info_header("INFO");
logger.set_warning_header("WARNING");
logger.set_error_header("ERROR");
logger.set_fatal_header("FATAL ERROR");
```

Log type headers can be further customized by changing their colors:
```rust
logger.set_debug_color(Color::Blue);
logger.set_info_color(Color::Green);
logger.set_warning_color(Color::Yellow);
logger.set_error_color(Color::Red);
logger.set_fatal_color(Color::Magenta);
```

The `Color` enum is declared in `prettylogger::colors`. It can hold one of the
following values:

- `None` **→** Represents no color. When a log type header color is set to this
value, it will appear as regular text.
- `Black` **→** Black color.
- `Blue` **→** The default color for **debug** header.
- `Cyan`
- `Green` **→** The default color for **info** header.
- `Gray`
- `Magenta` **→** The default color for **fatal** header.
- `Red` **→** The default color for **error** header.
- `White`
- `Yellow` **→** The default color for **warning** header.

<a name="log-filtering"></a>
## Log Filtering
Logs are filtered based on the current `LogLevel` and the `Logger`'s `Verbosity`
setting.

The `Verbosity` level determines which logs are filtered out:
- `All` **→** Disables log filtering, allowing all logs to pass through.
- `Standard` (default) **→** Filters out debug logs.
- `Quiet` **→** Only allows errors and warnings to be displayed.
- `ErrorsOnly` **→** Only allows errors to be shown.

The `Verbosity` enum is defined in `prettylogger::filtering`.

To modify the `Verbosity` of the `Logger`, use:
```rust
logger.set_verbosity(verbosity: Verbosity);
```

To toggle log filtering, use:
```rust
logger.toggle_log_filtering(enabled: bool);
```


<a name="file-logging"></a>
## File Logging
File logging is a feature that allows you to automatically save log output to a
file.

**Enabling file logging**:
```rust
// Set the log file path first:
logger.set_log_file_path("/path/to/file.log");
// Then enable file logging:
logger.toggle_file_logging(true);

logger.info("Yay!"); // Yay!
logger.flush(); // Flush the log buffer to a file
```

It is **CRUCIAL** to set the log file path **FIRST**. This is because when
you attempt to enable file logging, `Logger` will check if the log file path is
correct and since the default log file path is an empty string, you will get an
error.

<a name="file-logging_locking-the-log-file"></a>
### Locking the Log File
The log file can be locked to prevent race conditions when there are multiple
threads accessing it at the same time. The lock prevents `Logger` from writing to
the log file until the lock has been released. `Logger` only ignores the log file
lock when it's being dropped and the `OnDropPolicy` is set to `IgnoreLogFileLock`
(off by default).

**Note**: Log file lock is not persistent (it's not saved when calling
`logger.save_template("path")`).

To toggle log file lock, use:
```rust
logger.toggle_lock_file_lock(true);

// Do some I/O operations on the log file here

logger.toggle_lock_file_lock(false);
```

To set the on drop log file policy, use:
```rust
// Ignore the log file lock and write to the log file anyway.
logger.set_on_drop_file_policy(OnDropPolicy::IgnoreLogFileLock);
```

`OnDropPolicy` is declared in the `logging` module, and all it's possible values
are:
* `IgnoreLogFileLock` **→** Ignore the log file lock and write to the log file
anyway.
* `DiscardLogBuffer` (default) **→** Don't write to the log file on drop (discard
all logs from log buffer).

<a name="file-logging_automatic-log-buffer-flushing"></a>
### Automatic Log Buffer Flushing
You can either flush the log buffer automatically or set up automatic flushing
based on the log buffer size:
```rust
logger.set_log_file_path("/path/to/file.log");
logger.toggle_file_logging(true);

// This will make `Logger` to flush the log buffer every 16 logs:
logger.set_max_log_buffer_size(16);

let mut i = 0;
loop {
    logger.info("Yay!");
    i += 1;
    if i >= 16 {
        break;
    }
}
```


<a name="logger-templates"></a>
## Logger Templates
A **Logger Template** is a JSON file that defines the configuration of a
`Logger` struct. This allows you to easily manage and store logging settings in a
file.

Here’s an example of what a `Logger` struct looks like in JSON format:
```json
{
  "verbosity": "Standard",
  "filtering_enabled": true,
  "log_header_color_enabled": true,
  "debug_color": "Blue",
  "info_color": "Green",
  "warning_color": "Yellow",
  "error_color": "Red",
  "fatal_color": "Magenta",
  "debug_header": "DBG",
  "info_header": "INF",
  "warning_header": "WAR",
  "error_header": "ERR",
  "fatal_header": "FATAL",
  "log_format": "[%h] %m",
  "datetime_format": "%Y-%m-%d %H:%M:%S",
  "file_logging_enabled": false,
  "log_file_path": "",
  "log_buffer_max_size": 128,
  "on_drop_policy": "DiscardLogBuffer"
}
```

Deserializing `Logger` from a template file:
```rust
let mut logger = Logger::from_template("/path/to/template.json");
```

Serializing `Logger` to a template file:
```rust
let mut logger = Logger::default(); // Create a default `Logger`
logger.save_template("/path/to/template.json");
```
