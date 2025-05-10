![libprettylogger logo](https://raw.githubusercontent.com/tpaau-17DB/libprettylogger/main/img/libprettylogger-logo.png)

![CI Ubuntu](https://img.shields.io/github/actions/workflow/status/tpaau-17DB/libprettylogger/Ubuntu.yml?branch=main)
![Crates.io](https://img.shields.io/crates/v/libprettylogger.svg)

## Table of Contents
* [TL;DR](#tldr)
* [Installation](#installation)
* [Log Format](#log-format)
* [The Logger](#the-logger)
    * [Controlling Terminal Output](#the-logger_controlling-terminal-output)
    * [Custom Log Buffer](#the-logger_custom-log-buffer)
* [Log Filtering](#log-filtering)
* [File Logging](#file-logging)
    * [Automatic Log File Flushing](#file-logging_automatic-log-buffer-flushing)
    * [Locking the Log File](#file-logging_locking-the-log-file)
* [Logger Templates](#logger-templates)


<a name="tldr"></a>
## TL;DR
### Installing the library:
```bash
cargo add libprettylogger
```

### Quick start:
```rust
// Include stuff from the library:
use prettylogger::Logger;
use prettylogger::config::Verbosity;

// A `Logger` struct with default configuration:
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
```bash
cargo add libprettylogger
```

Or add this to your `Cargo.toml`:
```toml
[dependencies]
libprettylogger = "2.0.0"
```


<a name="the-logger"></a>
## The Logger
The `Logger` struct is the **core** of the entire project.
This is what you are going to use when you want to print a log, set filtering
rules or modify log formatting. All of it's fields are private, only allowing for
modification via setters.

Creating a `Logger` struct with default configuration:
```rust
# use prettylogger::Logger;
let mut logger = Logger::default();
```

<a name="the-logger_controlling-terminal-output"></a>
### Controlling Terminal Output
By default, `Logger` streams all logs to `stdout` and `stderr`. If you only want
to write logs to a file or store them in a custom buffer, use:
```rust
# use prettylogger::Logger;
# let mut logger = Logger::default();
logger.toggle_console_output(false);
```

<a name="the-logger_custom-log-buffer"></a>
### Custom Log Buffer
`Logger` can store logs in a buffer instead of printing them or writing them
to a file. Later, you can reference that buffer and do whatever you want with it.

Enabling custom log buffer:
```rust
# use prettylogger::Logger;
# let mut logger = Logger::default();
logger.toggle_custom_log_buffer(true);
```

And when you need a reference to that buffer, call:
```rust
# use prettylogger::Logger;
# let mut logger = Logger::default();
let buffer = logger.log_buffer();
```


<a name="log-format"></a>
## Log Format
A basic log consists of several headers:
* **Log Type** **→** The type of the log (debug, info, warning etc.)
* **Timestamp** **→** Contains the date and time the log was created
* **Message** **→** The actual log message

Those headers can then be formatted using a log format string, similarly to how
you would format a datetime string.

Here is a log message with all it's headers marked:
```markup
[ DEBUG 21:52:37 An example debug message ]
  ^^^^^ ^^^^^^^^ ^^^^^^^^^^^^^^^^^^^^^^^^
  |     |        |
  |     |        the message
  |     timestamp
  log type
```
This specific effect was achieved by setting the datetime format to `%H:%M:%S`,
log format to `[ %h %d %m ]` and the debug log type header to `DEBUG`.

Use this method to set the datetime format of a `Logger`:
```rust
# use prettylogger::Logger;
# let mut logger = Logger::default();
logger.set_datetime_format("%H:%M:%S");
```

You can set a custom log format like this:
```rust
# use prettylogger::Logger;
# let mut logger = Logger::default();
logger.set_log_format("[ %h %d %m ]");
```
**Note** that the `%m` (message) placeholder is mandatory and you will get an error
unless you include it in your format string.

Log type headers can be customized with their corresponding methods:
```rust
# use prettylogger::Logger;
# let mut logger = Logger::default();
logger.set_debug_header("DEBUG");
logger.set_info_header("INFO");
logger.set_warning_header("WARNING");
logger.set_error_header("ERROR");
logger.set_fatal_header("FATAL ERROR");
```

And you can also customize their colors:
```rust
# use prettylogger::{Logger, colors::Color};
# let mut logger = Logger::default();
logger.set_debug_color(Color::Blue);
logger.set_info_color(Color::Green);
logger.set_warning_color(Color::Yellow);
logger.set_error_color(Color::Red);
logger.set_fatal_color(Color::Magenta);
```

### Using the `LogStruct`
`LogStruct` is a type that represents a single log entry. You can create `LogStruct` instance using one of it's constructors:
* `debug(message: &str)`
* `info(message: &str)`
* `warning(message: &str)`
* `error(message: &str)`
* `fatal_error(message: &str)`

Using `LogStruct`'s `debug` constructor to create a debug log, and then formatting
with `logger.format_log(...)`:
```rust
# use prettylogger::{Logger, config::LogStruct};
# let mut logger = Logger::default();
let log_formatted = logger.format_log(&LogStruct::debug("A debug log!"));
```

<a name="log-filtering"></a>
## Log Filtering
Logs are filtered based on the current `LogLevel` and the `Logger`'s verbosity
setting. `config::Verbosity` enum can be used to set the verbosity of a `Logger`.

The `Verbosity` level determines which logs are filtered out:
- `All` **→** Disables log filtering, allowing all logs to pass through.
- `Standard` (default) **→** Filters out debug logs.
- `Quiet` **→** Only allows errors and warnings to be displayed.
- `ErrorsOnly` **→** Only allows errors to be shown.

To modify the verbosity of the `Logger`, use:
```rust
# use prettylogger::{Logger, config::Verbosity};
# let mut logger = Logger::default();
logger.set_verbosity(Verbosity::All);
```

To toggle log filtering, use:
```rust
# use prettylogger::{Logger, config::Verbosity};
# let mut logger = Logger::default();
logger.toggle_log_filtering(false);
```


<a name="file-logging"></a>
## File Logging
File logging is a feature that allows you to automatically save log output to a
file.

**Enabling file logging**:
```rust
# use prettylogger::Logger;
# let mut logger = Logger::default();
# let mut path = std::env::temp_dir();
# path.push("libprettylogger-tests/readme-doc1.log");
# let path = &path.to_str().unwrap().to_string();

// Set the log file path first:
logger.set_log_file_path(path);
// Then enable file logging:
logger.toggle_file_logging(true);

logger.info("Yay!"); // Yay!
logger.flush(); // Flush the log buffer to a file
```

It is **CRUCIAL** to set the log file path **FIRST**. If you try to enable file
logging before specifying a valid path, `Logger` will check the log file path, and
since the default path is an empty string, it will result in an error.


<a name="file-logging_locking-the-log-file"></a>
### Locking the Log File
The log file can be locked to prevent race conditions when there are multiple
threads accessing it at the same time. It prevents `Logger` from writing to
the log file until the lock has been released. The lock is only ignored when a
`Logger` is being dropped and the `OnDropPolicy` is set to `IgnoreLogFileLock`
(off by default).

Log file lock is not persistent (it's not saved when calling
`logger.save_template("path")`).

Toggling log file lock:
```rust
# use prettylogger::Logger;
# let mut logger = Logger::default();
logger.toggle_log_file_lock(true);
// Do some I/O operations on the log file here
logger.toggle_log_file_lock(false);
```

To set the on drop log file policy, use:
```rust
# use prettylogger::{Logger, config::OnDropPolicy};
# let mut logger = Logger::default();
logger.set_on_drop_file_policy(OnDropPolicy::IgnoreLogFileLock);
```


<a name="file-logging_automatic-log-buffer-flushing"></a>
### Automatic Log Buffer Flushing
You can either flush the log buffer automatically or set up automatic flushing
based on the log buffer size:
```rust
# use prettylogger::Logger;
# let mut logger = Logger::default();
# let mut path = std::env::temp_dir();
# path.push("libprettylogger-tests/readme-doc2.log");
# let path = &path.to_str().unwrap().to_string();
logger.set_log_file_path(path);
logger.toggle_file_logging(true);

// This will make `Logger` to flush the log buffer every 16 logs:
logger.set_max_log_buffer_size(16 as u32);

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
A **Logger template** is a JSON file that defines the configuration of a
`Logger` struct. This allows you to easily manage and store logging settings in a
file.

Here’s an example of what a `Logger` struct looks like in JSON format:
```json
{
  "console_out_enabled": true,
  "use_custom_log_buffer": false,
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
  "file_log_buffer_max_size": 128,
  "on_drop_policy": "DiscardLogBuffer"
}
```

Loading `Logger` from a template file:
```rust
# use prettylogger::Logger;
# let mut path = std::env::temp_dir();
# path.push("libprettylogger-tests/readme-doc3.log");
# let path = &path.to_str().unwrap().to_string();
# Logger::default().save_template(path);
let mut logger = Logger::from_template(path);
```

Deserializing `Logger` from a JSON string:
```rust
# use prettylogger::Logger;
let raw_json = serde_json::to_string(&Logger::default())
    .expect("Failed to serialize logger!");
# assert_eq!(Logger::default(), Logger::from_template_str(&raw_json)
    .expect("Failed to deserialize logger!"));
```

Saving `Logger` to a template file:
```rust
# use prettylogger::Logger;
let mut logger = Logger::default(); // Create a default `Logger`
# let mut path = std::env::temp_dir();
# path.push("libprettylogger-tests/readme-doc4.log");
# let path = &path.to_str().unwrap().to_string();
logger.save_template(path);
```
