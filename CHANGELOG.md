# v2.0.0

## Minor changes
* Added a new function in `colors` module that colors text using ANSII escape
codes.

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
