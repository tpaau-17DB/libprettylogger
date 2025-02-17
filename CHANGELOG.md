# v2.0.0

## Major changes
* Merged `filtering` with some types from `logging` as `config`.
* Moved `prettylogger::logging::Logger` to `prettylogger::Logger`.

## Minor changes
* Added a new function in `colors` module that colors text using ANSII escape
codes.
* Added a new option in `Logger` to toggle logging to `stdout`.
* Added a `print_log(...)` method in `Logger` that prints logs from `LogStruct`
instances.
* Added a `format_log(...)` method in `Logger` that formats logs from `LogStruct`
instances.
* Added a new `Logger` feature that allows you to store logs in a log buffer
inside `Logger` for later use.
* Added 5 new constructors to the `LogStruct`, each one representing a different
log type.

## Patches
* Changed `prettylogger::setters` module accessibility to private.
* Fixed a bug where log filtering toggle would not work as expected.


# v1.0.0

## Major changes:
* Changed some of the arguments accepted by `Logger`'s setter methods from
references to owned values.

## Patches:
* Introduced automatic argument casting for some of `Logger`'s setter methods ([C_GENERIC](https://rust-lang.github.io/api-guidelines/flexibility.html#c-generic)).
* Updated some of `Logger`'s methods and functions.
* Updated `Logger`'s default constructor.
