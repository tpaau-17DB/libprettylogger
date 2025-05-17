<h1 align="center">
v3.0.0
</h1>

## Major changes
* Refactored log formatting system
* Refactored log outputting system
* Removed `_no_filtering` logging methods
* Split `Logger`s `toggle_log_filtering(...)` into two methods
* Removed `print_log(...)` method from logger

## Minor changes
* Removed automatic tilde and environment variable expansion for extended
flexibility
* Updated dependencies to their latest versions

## Patches
* Removed the `default` constructor from `LogStruct`
* All logs now go to `stderr`
* Removed bright color variants from `Color`
* Added a new color variant that allows for specifying custom ANSII escape
codes


<h1 align="center">
v2.1.0
</h1>

## Minor changes
* Added a new constructor to create a `Logger` instance from a JSON string

## Patches
* Fixed various issues and typos in error messages
* Fixed various issues and typos in docstrings
* `Logger::flush()` now returns an error when log file lock is enabled
* `Logger::from_template()` and `logger.save_template(...)` now return
`Result<...>` enums instead of panicking
* Warnings now go to `stderr` instead of `stdout`.
* Updated the codebase to follow [my style](https://github.com/tpaau-17DB/coding-style/blob/main/src/RUST.md)


<h1 align="center">
v2.0.0
</h1>

## Major changes
* Merged `filtering` with some types from `logging` as `config`
* Moved `prettylogger::logging::Logger` to `prettylogger::Logger`

## Minor changes
* Added a new function in `colors` module that colors text using ANSII escape
codes
* Added a new option in `Logger` to toggle logging to terminal
* Added a `print_log(...)` method in `Logger` that prints logs from `LogStruct`
instances
* Added a `format_log(...)` method in `Logger` that formats logs from
`LogStruct` instances.
* Added a new `Logger` feature that allows you to store logs in a log buffer
inside `Logger` for later use.
* Added 5 new constructors to the `LogStruct`, each one returning a different
log type.
* `Logger`'s `from_template()` constructor now automatically expands environment
variables
* `Logger`'s `save_template(...)` and `set_log_file_path(...)` methods now
automatically expand environment variables
* `Logger` now prints debug, info and warning messages to `stdout` and error
messages to `stderr`

## Patches
* Changed `prettylogger::setters` module accessibility to private
* Fixed a bug where log filtering toggle would not work as expected
* Fixed a bug where `Logger` would not add a timestamp when loaded from a
template


<h1 align="center">
v1.0.0
</h1>

## Major changes:
* Changed some of the arguments accepted by `Logger`'s setter methods from
references to owned values

## Patches:
* Introduced automatic argument casting for some of `Logger`'s setter methods
([C_GENERIC](https://rust-lang.github.io/api-guidelines/flexibility.html#c-generic))
* Updated some of `Logger`'s methods and functions
* Updated `Logger`'s default constructor
