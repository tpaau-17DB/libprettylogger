# v2.0.0

## Major changes
* Merged `filtering` with some types from `logging` as `config`.
* Moved `prettylogger::logging::Logger` to `prettylogger::Logger`.

## Minor changes
* Added a new function in `colors` module that can be used to color text using
ANSII escape codes.
* Added an option to toggle logging to `stdout`.

## Patches
* Changed `prettylogger::setters` module accessibility to private.


# v1.0.0

## Major changes:
* Changed some of the arguments accepted by `Logger`'s setter methods from
references to owned values.

## Patches:
* Introduced automatic argument casting for some of `Logger`'s setter methods ([C_GENERIC](https://rust-lang.github.io/api-guidelines/flexibility.html#c-generic)).
* Updated some of `Logger`'s methods and functions.
* Updated `Logger`'s default constructor.
