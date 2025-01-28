use crate::logging::*;
use crate::utils::formatting::*;

pub fn print_log(log: &LogStruct) {
    print!("{}", format_log(log));
}
