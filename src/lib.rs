pub mod logging;
mod utils;

#[cfg(test)]
mod tests {
    use crate::logging::*;
    #[test]
    fn it_works() {
        debug(&"Debug log.".to_string());
        info(&"Informative log.".to_string());
        warn(&"A warning.".to_string());
        err(&"An error.".to_string());
        fatal(&"A fatal error.".to_string());
    }
}
