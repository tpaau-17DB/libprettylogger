![libprettylogger logo](https://raw.githubusercontent.com/tpaau-17DB/libprettylogger/main/img/libprettylogger-logo.png)

![CI Ubuntu](https://img.shields.io/github/actions/workflow/status/tpaau-17DB/libprettylogger/Ubuntu.yml?branch=main)
![Crates.io](https://img.shields.io/crates/v/libprettylogger.svg)

## Table of Contents
* [TL;DR](#tldr)
* [The Logger](#the-logger)
* [Log Filtering](#log-filtering)
* [Log Format](#log-format)
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
use prettylogger::Logger;
use prettylogger::config::Verbosity;

// A `Logger` struct with default configuration
let mut logger = Logger::default();

// Don't suppress any log messages
logger.set_verbosity(Verbosity::All);

// Print logs
logger.debug("A debug message!");
logger.info("Info message!");
logger.warning("A warning!");
logger.error("An error!");
logger.fatal("A fatal error!");
```

<a name="the-logger"></a>
## The Logger
The `Logger` struct only handles log filtering, relying on `LogFormatter` and
`LogOutput` for formatting and outputting the logs.

Creating a `Logger` struct with default configuration:
```rust
# use prettylogger::Logger;
let mut logger = Logger::default();
```

<a name="the-logger_log-filtering"></a>
## Log Filtering
Logs are filtered based on their importance and the `Logger`'s verbosity
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

<a name="the-logger_logger-templates"></a>
### Logger Templates
A **Logger template** is just a JSON file that defines the configuration of a
`Logger` struct. This can be used to easily manage and store logger
configurations in files.

Here’s an example of what a `Logger` struct looks like in JSON format:
```json
{
  "formatter": {
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
    "datetime_format": "%Y-%m-%d %H:%M:%S"
  },
  "output": {
    "stderr_output": {
      "enabled": true
    },
    "file_output": {
      "enabled": false,
      "max_buffer_size": 128,
      "on_drop_policy": "DiscardLogBuffer"
    },
    "buffer_output": {
      "enabled": false
    },
    "enabled": true
  },
  "verbosity": "Standard",
  "filtering_enabled": true
}
```

Loading `Logger` from a template file:
```rust
# use prettylogger::Logger;
# let mut path = std::env::temp_dir();
# path.push("libprettylogger-tests/readme-doc1.log");
# let path = path.to_str().unwrap().to_string();
# Logger::default().save_template(&path);
let mut logger = Logger::from_template(&path);
```

Deserializing `Logger` from a JSON string:
```rust
# use prettylogger::Logger;
let raw_json = serde_json::to_string(&Logger::default())
    .expect("Failed to serialize logger!");
assert_eq!(Logger::default(), Logger::from_template_str(&raw_json)
   .expect("Failed to deserialize logger!"));
```

Saving `Logger` to a template file:
```rust
# use prettylogger::Logger;
# let mut path = std::env::temp_dir();
# path.push("libprettylogger-tests/readme-doc2.log");
# let path = &path.to_str().unwrap().to_string();
let mut logger = Logger::default();
logger.save_template(path);
```

<a name="log-formatting"></a>
## Log Formatting

<a name="log-formatting_log-formatter"></a>
### LogFormatter
The `LogFormatter` struct manages log formatting. It's accessible as a field
within the `Logger`, but can also operate independently. This means that
`LogFormatter` can be used directly without the need for a `Logger` instance.

Using a `LogFormatter`
```rust
# use prettylogger::{
#    config::LogStruct,
#    format::LogFormatter,
# };
// Create a `LogFormatter` with default configuration
let mut formatter = LogFormatter::default();

// Set a log format
formatter.set_log_format("[ %h %m ]");

// Obtain a formatted log from a `LogStruct`
let log = formatter.format_log(&LogStruct::debug("Hello from LogStruct!"));

// Print the formatted log message
print!("{}", &log);
```

<a name="log-formatting_log-format"></a>
### Log Format
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

Setting datetime format of a `LogFormatter`
```rust
# use prettylogger::format::LogFormatter;
# let mut formatter = LogFormatter::default();
formatter.set_datetime_format("%H:%M:%S");
```

Setting a custom log format
```rust
# use prettylogger::format::LogFormatter;
# let mut formatter = LogFormatter::default();
formatter.set_log_format("[ %h %d %m ]");
```
Note that the `%m` (message) placeholder is mandatory. You will get an error
unless you include it in your format string.

Customizing log headers
```rust
# use prettylogger::format::LogFormatter;
# let mut formatter = LogFormatter::default();
formatter.set_debug_header("DEBUG");
formatter.set_info_header("INFO");
formatter.set_warning_header("WARNING");
formatter.set_error_header("ERROR");
formatter.set_fatal_header("FATAL ERROR");
```

Setting custom log header colors:
```rust
# use prettylogger::{
#     format::LogFormatter,
#     colors::Color
# };
# let mut formatter = LogFormatter::default();
formatter.set_debug_color(Color::Blue);
formatter.set_info_color(Color::Green);
formatter.set_warning_color(Color::Yellow);
formatter.set_error_color(Color::Red);
formatter.set_fatal_color(Color::Magenta);
```

### Using the `LogStruct`
`LogStruct` is a type that represents a single log entry. You can create
`LogStruct` instance using one of it's constructors:
* `debug(message: &str)`
* `info(message: &str)`
* `warning(message: &str)`
* `error(message: &str)`
* `fatal_error(message: &str)`

Creating a `LogStruct` and formatting it with a `LogFormatter`:
```rust
# use prettylogger::{
#     format::LogFormatter,
#     config::LogStruct
# };
# let mut formatter = LogFormatter::default();
// Create a `LogStruct`
let raw_log = LogStruct::debug("Hello from a struct!");

// Format the `LogStruct`
let formatted_log = formatter.format_log(&raw_log);

// Print the formatted log
print!("{}", &formatted_log);
```


<a name="log-outputs"></a>
## Log Outputs
Log outputs determine how messages are routed, delivering logs to specific
destinations  like standard error (`StderrStream`) or a dedicated log file
(`FileStream`). Each output can be selectively toggled. Additionally, the
parent output provides an overall control mechanism; disabling it effectively
halts all child streams.

<a name="log-outputs_log-output"></a>
### `LogOutput` (parent)

<a name="log-outputs_stderr-stream"></a>
### `StderrStream`
This is the simplest of the log outputs. It simply formats the given log using
the formatter and prints it to `stderr`.

Using `StderrStream`:
```rust
# use prettylogger::{
#     output::StderrStream,
#     format::LogFormatter,
#     config::LogStruct,
# };
let stderr_stream = StderrStream::default();
```

<a name="log-outputs_buffer-stream"></a>
### `BufferStream`
When enabled, `BufferStream` stores logs in an internal buffer in raw format.

Using `BufferStream`:
```rust
# use prettylogger::{
#     output::BufferStream,
#     output::Toggleable,
#     config::LogStruct,
# };
let mut buffer_stream = BufferStream::default();

// Enable the buffer stream
buffer_stream.enable();

// Write to the buffer 128 times
for i in 0..128 {
    buffer_stream.out(&LogStruct::debug(&format!("Log number {}", i)));
}

// Get a reference to the log buffer
let buffer = buffer_stream.get_log_buffer();
# assert_eq!(buffer.len(), 128);

// Do whatever you wish with the log buffer here

// Clear the log buffer
buffer_stream.clear();
```

<a name="log-outputs_file-stream"></a>
### `FileStream`
File stream is used for storing logs in a log file. For performance reasons,
`FileStream` utilizes an internal log buffer for storing already formatted
log messages until they are written to the log file.

#### Using `FileStream`
```rust
# use prettylogger::{
#     config::LogStruct,
#     format::LogFormatter,
#     output::{Toggleable, FileStream},
# };
# let mut path = std::env::temp_dir();
# path.push("libprettylogger-tests/readme-file-stream-doc1.log");
# let path = &path.to_str().unwrap().to_string();
let formatter = LogFormatter::default();

let mut file_stream = FileStream::default();

// Set the log file path before enabling the stream
file_stream.set_log_file_path(&path)
    .expect("Failed setting log file path!");

// Enable the stream after the log file path has been set
file_stream.enable()
    .expect("Failed enabling the file stream!");

// Write to the log buffer
file_stream.out(&LogStruct::debug("Hello from a file!"), &formatter)
    .expect("Failed outing to the file stream!");

// Write the contents of the log buffer to the log file
file_stream.flush()
    .expect("Failed flushing the file stream!");
```

Note that log file path has to be set in order to enable the file stream.

#### Automatic Log File Flushing
`FileStream` can automatic write to the log file when it reaches a specific
limit. This limit can either be set to a `Some` value, meaning that the log
buffer will be automatic flushed, or to `None` to disable automatic flushing.

```rust
# use prettylogger::{
#     output::{FileStream, Toggleable},
#     format::LogFormatter,
#     config::LogStruct,
# };
# let mut path = std::env::temp_dir();
# path.push("libprettylogger-tests/readme-file-stream-doc2.log");
# let path = &path.to_str().unwrap().to_string();
let formatter = LogFormatter::default();

let mut file_stream = FileStream::default();
file_stream.set_log_file_path(&path)
    .expect("Failed setting log file path!");
file_stream.enable()
    .expect("Failed enabling the file stream!");

file_stream.set_max_buffer_size(Some(128));

for i in 0..128 {
    file_stream.out(&LogStruct::debug("Hello!"), &formatter)
        .expect("Failed to out to the log buffer!");
}
// Here the log buffer will be automatically flushed.
```

#### Locking The Log File
The log file can be locked to prevent race conditions when there are multiple
threads accessing it at the same time. It prevents `FileStream` from writing to
the log file until the lock has been released. The lock is only ignored when the
`FileStream` is being dropped and the `OnDropPolicy` is set to
`IgnoreLogFileLock` (off by default).

Toggling the lock:
```rust
# use prettylogger::output::FileStream;
# let mut file_stream = FileStream::default();
// Lock the log file
file_stream.lock_file();

// Unlock the log file
file_stream.unlock_file();
```

Setting on drop policy:
```rust
# use prettylogger::{
#     output::FileStream,
#     config::OnDropPolicy,
# };
# let mut file_stream = FileStream::default();
file_stream.set_on_drop_policy(OnDropPolicy::IgnoreLogFileLock);
```
